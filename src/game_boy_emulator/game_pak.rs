// Copyright 2019 Remi Bernotavicius

use std::cell::RefCell;
use std::fmt;
use std::ops::Range;
use std::rc::Rc;
use std::str;

use super::memory_controller::{GameBoyMemoryMap, MappingType, MemoryChunk, MemoryMappedHardware};

trait MemoryBankController: fmt::Debug {
    fn map(&self, ma: &mut GameBoyMemoryMap);
    fn tick(&mut self) {}
}

struct MemoryBankController0 {
    banks: Vec<MemoryChunk>,
}

impl MemoryBankController0 {
    fn new(banks: Vec<MemoryChunk>) -> Self {
        MemoryBankController0 { banks }
    }
}

impl fmt::Debug for MemoryBankController0 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MBC0")
    }
}

impl MemoryBankController for MemoryBankController0 {
    fn map(&self, ma: &mut GameBoyMemoryMap) {
        ma.map_chunk(0, self.banks[0].clone(), MappingType::Read);
        ma.map_chunk(0x4000, self.banks[1].clone(), MappingType::Read);
    }
}

struct SwitchableBankInner {
    banks: Vec<MemoryChunk>,
    current_bank: usize,
}

#[derive(Clone)]
struct SwitchableBank {
    banks: Rc<RefCell<SwitchableBankInner>>,
}

impl SwitchableBank {
    fn new(banks: Vec<MemoryChunk>) -> Self {
        SwitchableBank {
            banks: Rc::new(RefCell::new(SwitchableBankInner {
                banks,
                current_bank: 1,
            })),
        }
    }

    fn switch_bank(&mut self, new_bank: usize) {
        let mut inner = self.banks.borrow_mut();
        let new_bank = match new_bank {
            0 => 1,
            v if v > inner.banks.len() => panic!("Switch to non-existent bank {}", v),
            v => v,
        };
        inner.current_bank = new_bank;
    }

    fn clone_bank(&self, number: usize) -> MemoryChunk {
        self.banks.borrow().banks[number].clone()
    }
}

impl MemoryMappedHardware for SwitchableBank {
    fn read_value(&self, address: u16) -> u8 {
        let current_bank = self.banks.borrow().current_bank;
        self.banks.borrow().banks[current_bank].read_value(address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        let current_bank = self.banks.borrow().current_bank;
        self.banks.borrow_mut().banks[current_bank].set_value(address, value)
    }

    fn len(&self) -> u16 {
        BANK_SIZE
    }
}

#[derive(Clone)]
struct RangeRegister {
    value: Rc<RefCell<u8>>,
    size: u16,
}

impl RangeRegister {
    fn new(size: u16) -> Self {
        RangeRegister {
            value: Rc::new(RefCell::new(0)),
            size,
        }
    }

    fn value(&self) -> u8 {
        *self.value.borrow()
    }
}

impl MemoryMappedHardware for RangeRegister {
    fn read_value(&self, _address: u16) -> u8 {
        panic!();
    }

    fn set_value(&mut self, _address: u16, value: u8) {
        *self.value.borrow_mut() = value;
    }

    fn len(&self) -> u16 {
        self.size
    }
}

struct MemoryBankController1<R: CartridgeRam> {
    rom_bank_number: RangeRegister,
    ram_bank_number: RangeRegister,
    rom_ram_select: RangeRegister,
    ram_enable: RangeRegister,
    switchable_bank: SwitchableBank,
    ram: R,
}

impl<R: CartridgeRam> fmt::Debug for MemoryBankController1<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MBC1{:?}", self.ram)
    }
}

impl<R: CartridgeRam> MemoryBankController1<R> {
    fn new(banks: Vec<MemoryChunk>, ram: R) -> Self {
        let rom_bank_number = RangeRegister::new(0x2000);
        let ram_bank_number = RangeRegister::new(0x2000);
        let rom_ram_select = RangeRegister::new(0x2000);
        let ram_enable = RangeRegister::new(0x2000);
        MemoryBankController1 {
            rom_bank_number,
            ram_bank_number,
            rom_ram_select,
            ram_enable,
            switchable_bank: SwitchableBank::new(banks),
            ram,
        }
    }
}

impl<R: CartridgeRam> MemoryBankController for MemoryBankController1<R> {
    fn map(&self, ma: &mut GameBoyMemoryMap) {
        ma.map_chunk(0, self.switchable_bank.clone_bank(0), MappingType::Read);
        ma.map_chunk(0, self.ram_bank_number.clone(), MappingType::Write);
        ma.map_chunk(0x2000, self.rom_bank_number.clone(), MappingType::Write);
        ma.map_chunk(0x4000, self.ram_bank_number.clone(), MappingType::Write);
        ma.map_chunk(0x4000, self.switchable_bank.clone(), MappingType::Read);
        ma.map_chunk(0x6000, self.rom_ram_select.clone(), MappingType::Write);
        self.ram.map(ma);
    }

    fn tick(&mut self) {
        let mut rom_bank_value = (self.rom_bank_number.value() & 0x0F) as usize;

        if self.rom_ram_select.value() == 0 {
            rom_bank_value |= ((self.ram_bank_number.value() & 0b00000011) as usize) << 4;
        }

        self.switchable_bank.switch_bank(rom_bank_value);

        if self.rom_ram_select.value() != 0 && self.ram_enable.value() & 0x0F == 0x0A {
            self.ram.switch_bank(self.ram_bank_number.value() as usize);
        }
    }
}

struct MemoryBankController5<R: CartridgeRam> {
    inner: MemoryBankController1<R>,
}

impl<R: CartridgeRam> fmt::Debug for MemoryBankController5<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MBC5{:?}", self.inner.ram)
    }
}

impl<R: CartridgeRam> MemoryBankController5<R> {
    fn new(banks: Vec<MemoryChunk>, ram: R) -> Self {
        MemoryBankController5 {
            inner: MemoryBankController1::new(banks, ram),
        }
    }
}

impl<R: CartridgeRam> MemoryBankController for MemoryBankController5<R> {
    fn map(&self, ma: &mut GameBoyMemoryMap) {
        self.inner.map(ma);
    }

    fn tick(&mut self) {
        self.inner.tick();
    }
}

trait CartridgeRam: fmt::Debug {
    fn map(&self, ma: &mut GameBoyMemoryMap);
    fn switch_bank(&mut self, bank: usize);
}

struct NoRam;

impl fmt::Debug for NoRam {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl CartridgeRam for NoRam {
    fn map(&self, _: &mut GameBoyMemoryMap) {}
    fn switch_bank(&mut self, _bank: usize) {}
}

struct VolatileRam {
    switchable_bank: SwitchableBank,
}

impl fmt::Debug for VolatileRam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+RAM")
    }
}

impl VolatileRam {
    fn new(banks: Vec<MemoryChunk>) -> Self {
        VolatileRam {
            switchable_bank: SwitchableBank::new(banks),
        }
    }
}

impl CartridgeRam for VolatileRam {
    fn map(&self, ma: &mut GameBoyMemoryMap) {
        ma.map_chunk(0xA000, self.switchable_bank.clone(), MappingType::ReadWrite);
    }

    fn switch_bank(&mut self, bank: usize) {
        self.switchable_bank.switch_bank(bank)
    }
}

struct NonVolatileRam {
    _inner: VolatileRam,
}

impl fmt::Debug for NonVolatileRam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+RAM+BATTERY")
    }
}

impl NonVolatileRam {
    fn new(banks: Vec<MemoryChunk>) -> Self {
        NonVolatileRam {
            _inner: VolatileRam::new(banks),
        }
    }
}

impl CartridgeRam for NonVolatileRam {
    fn map(&self, _: &mut GameBoyMemoryMap) {}
    fn switch_bank(&mut self, _bank: usize) {}
}

pub struct GamePak {
    title: String,
    mbc: Box<MemoryBankController>,
}

const BANK_SIZE: u16 = 0x4000;
const MBC_TYPE_ADDRESS: usize = 0x0147;
const ROM_SIZE_ADDRESS: usize = 0x0148;
const RAM_SIZE_ADDRESS: usize = 0x0149;
const TITLE: Range<usize> = Range {
    start: 0x0134,
    end: 0x0144,
};

impl GamePak {
    pub fn from(rom: &[u8]) -> Self {
        assert_eq!(rom.len() % (BANK_SIZE as usize), 0, "ROM wrong size");
        let number_of_banks = match rom[ROM_SIZE_ADDRESS] {
            n if n <= 0x08 => 2usize.pow(n as u32 + 1),
            0x52 => 72,
            0x53 => 80,
            0x54 => 96,
            v => panic!("Unknown ROM size {}", v),
        };
        assert_eq!(
            number_of_banks,
            rom.len() / (BANK_SIZE as usize),
            "ROM wrong size"
        );

        let title_slice = &rom[TITLE];
        let title_end = title_slice
            .iter()
            .position(|&c| c == '\0' as u8)
            .expect(&format!("Malformed title {:?}", title_slice));
        let title = str::from_utf8(&title_slice[..title_end])
            .expect(&format!("Malformed title {:?}", title_slice))
            .into();

        let mut banks = Vec::new();
        for b in 0..number_of_banks {
            let start = b * (BANK_SIZE as usize);
            let end = start + (BANK_SIZE as usize);
            banks.push(MemoryChunk::new(rom[start..end].to_vec()));
        }

        let ram = match rom[RAM_SIZE_ADDRESS] {
            0 => Vec::new(),
            // 2kB 1 Bank
            1 => vec![MemoryChunk::from_range(0..0x800)],
            // 8kB 1 Bank
            2 => vec![MemoryChunk::from_range(0..0x2000)],
            // 8kB 4 Banks = 32kB
            3 => vec![
                MemoryChunk::from_range(0..0x2000),
                MemoryChunk::from_range(0..0x2000),
                MemoryChunk::from_range(0..0x2000),
                MemoryChunk::from_range(0..0x2000),
            ],
            v => panic!("Unknown RAM size {}", v),
        };

        /*
         *  0x00  ROM ONLY
         *  0x01  MBC1
         *  0x02  MBC1+RAM
         *  0x03  MBC1+RAM+BATTERY
         *  0x05  MBC2
         *  0x06  MBC2+BATTERY
         *  0x08  ROM+RAM
         *  0x09  ROM+RAM+BATTERY
         *  0x0B  MMM01
         *  0x0C  MMM01+RAM
         *  0x0D  MMM01+RAM+BATTERY
         *  0x0F  MBC3+TIMER+BATTERY
         *  0x10  MBC3+TIMER+RAM+BATTERY
         *  0x11  MBC3
         *  0x12  MBC3+RAM
         *  0x13  MBC3+RAM+BATTERY
         *  0x19  MBC5
         *  0x1A  MBC5+RAM
         *  0x1B  MBC5+RAM+BATTERY
         *  0x1C  MBC5+RUMBLE
         *  0x1D  MBC5+RUMBLE+RAM
         *  0x1E  MBC5+RUMBLE+RAM+BATTERY
         *  0x20  MBC6
         *  0x22  MBC7+SENSOR+RUMBLE+RAM+BATTERY
         *  0xFC  POCKET CAMERA
         *  0xFD  BANDAI TAMA5
         *  0xFE  HuC3
         *  0xFF  HuC1+RAM+BATTERY
         */

        let mbc: Box<MemoryBankController> = match rom[MBC_TYPE_ADDRESS] {
            0x00 => {
                assert_eq!(number_of_banks, 2);
                Box::new(MemoryBankController0::new(banks))
            }
            0x01 => Box::new(MemoryBankController1::new(banks, NoRam)),
            0x02 => Box::new(MemoryBankController1::new(banks, VolatileRam::new(ram))),
            0x03 => Box::new(MemoryBankController1::new(banks, NonVolatileRam::new(ram))),
            0x1b => Box::new(MemoryBankController5::new(banks, NonVolatileRam::new(ram))),
            v => panic!("Unknown Memory Bank Controller #{:x}", v),
        };

        GamePak { title, mbc }
    }

    pub fn map(&mut self, ma: &mut GameBoyMemoryMap) {
        self.mbc.map(ma)
    }

    pub fn tick(&mut self) {
        self.mbc.tick();
    }
}

impl fmt::Debug for GamePak {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GamePak({:?}, {:?})", self.title, self.mbc)
    }
}
