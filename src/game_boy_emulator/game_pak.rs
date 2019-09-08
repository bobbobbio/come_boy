// Copyright 2019 Remi Bernotavicius

use super::memory_controller::{MemoryChunk, MemoryMappedHardware};
use std::fmt;
use std::ops::Range;
use std::str;

trait MemoryBankController: fmt::Debug + MemoryMappedHardware {
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

impl MemoryBankController for MemoryBankController0 {}

impl MemoryMappedHardware for MemoryBankController0 {
    fn read_value(&self, address: u16) -> u8 {
        if address < 0x4000 {
            self.banks[0].read_value(address)
        } else if address < 0x8000 {
            self.banks[1].read_value(address - 0x4000)
        } else {
            0xFF
        }
    }

    fn set_value(&mut self, _address: u16, _value: u8) {}
}

struct SwitchableBank {
    banks: Vec<MemoryChunk>,
    current_bank: usize,
}

impl SwitchableBank {
    fn new(banks: Vec<MemoryChunk>) -> Self {
        SwitchableBank {
            banks,
            current_bank: 0,
        }
    }

    fn switch_bank(&mut self, new_bank: usize) {
        let new_bank = match new_bank {
            0 => 1,
            v if v > self.banks.len() => panic!("Switch to non-existent bank {}", v),
            v => v,
        };
        self.current_bank = new_bank;
    }

    fn len(&self) -> usize {
        self.banks.len()
    }
}

impl MemoryMappedHardware for SwitchableBank {
    fn read_value(&self, address: u16) -> u8 {
        if self.current_bank >= self.banks.len() {
            0xFF
        } else {
            self.banks[self.current_bank].read_value(address)
        }
    }

    fn set_value(&mut self, address: u16, value: u8) {
        if self.current_bank < self.banks.len() {
            self.banks[self.current_bank].set_value(address, value);
        }
    }
}

#[derive(Clone)]
struct RangeRegister {
    value: u8,
    size: u16,
}

impl RangeRegister {
    fn new(size: u16) -> Self {
        RangeRegister { value: 0, size }
    }

    fn value(&self) -> u8 {
        self.value
    }
}

impl MemoryMappedHardware for RangeRegister {
    fn read_value(&self, _address: u16) -> u8 {
        panic!();
    }

    fn set_value(&mut self, _address: u16, value: u8) {
        self.value = value;
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

impl<R: CartridgeRam> MemoryMappedHardware for MemoryBankController1<R> {
    fn read_value(&self, address: u16) -> u8 {
        if address < 0x4000 {
            self.switchable_bank.banks[0].read_value(address)
        } else if address < 0xA000 {
            self.switchable_bank.read_value(address - 0x4000)
        } else {
            self.ram.read_value(address - 0xA000)
        }
    }

    fn set_value(&mut self, address: u16, value: u8) {
        if address < 0x2000 {
            self.ram_bank_number.set_value(address, value);
        } else if address < 0x4000 {
            self.rom_bank_number.set_value(address - 0x2000, value);
        } else if address < 0x6000 {
            self.ram_bank_number.set_value(address - 0x4000, value);
        } else if address < 0xA000 {
            self.rom_bank_number.set_value(address - 0x6000, value);
        } else {
            self.ram.set_value(address - 0xA000, value);
        }
    }
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
    fn tick(&mut self) {
        let mut rom_bank_value = (self.rom_bank_number.value() & 0x0F) as usize;

        // ROM bank zero cannot be selected, and this extends to higher numbered banks that can be
        // selected below using the ram_bank_number (0x20, 0x40, 0x60) => (0x21, 0x41, 0x61).
        if rom_bank_value == 0 {
            rom_bank_value += 1;
        }

        if self.rom_ram_select.value() == 0 {
            rom_bank_value |= ((self.ram_bank_number.value() & 0b00000011) as usize) << 4;
        }

        // XXX remi: I'm not sure whats suppose to happen if a non-existent ROM bank is requested.
        rom_bank_value %= self.switchable_bank.len();
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

impl<R: CartridgeRam> MemoryMappedHardware for MemoryBankController5<R> {
    fn read_value(&self, address: u16) -> u8 {
        self.inner.read_value(address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        self.inner.set_value(address, value);
    }
}

impl<R: CartridgeRam> MemoryBankController for MemoryBankController5<R> {
    fn tick(&mut self) {
        self.inner.tick();
    }
}

trait CartridgeRam: fmt::Debug + MemoryMappedHardware {
    fn switch_bank(&mut self, bank: usize);
}

struct NoRam;

impl fmt::Debug for NoRam {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl MemoryMappedHardware for NoRam {
    fn read_value(&self, _address: u16) -> u8 {
        0xFF
    }
    fn set_value(&mut self, _address: u16, _value: u8) {}
}

impl CartridgeRam for NoRam {
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

impl MemoryMappedHardware for VolatileRam {
    fn read_value(&self, address: u16) -> u8 {
        self.switchable_bank.read_value(address)
    }
    fn set_value(&mut self, address: u16, value: u8) {
        self.switchable_bank.set_value(address, value);
    }
}

impl CartridgeRam for VolatileRam {
    fn switch_bank(&mut self, bank: usize) {
        self.switchable_bank.switch_bank(bank)
    }
}

struct NonVolatileRam {
    inner: VolatileRam,
}

impl fmt::Debug for NonVolatileRam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+RAM+BATTERY")
    }
}

impl NonVolatileRam {
    fn new(banks: Vec<MemoryChunk>) -> Self {
        NonVolatileRam {
            inner: VolatileRam::new(banks),
        }
    }
}

impl MemoryMappedHardware for NonVolatileRam {
    fn read_value(&self, address: u16) -> u8 {
        self.inner.read_value(address)
    }
    fn set_value(&mut self, address: u16, value: u8) {
        self.inner.set_value(address, value);
    }
}

impl CartridgeRam for NonVolatileRam {
    fn switch_bank(&mut self, _bank: usize) {}
}

pub struct GamePak {
    title: String,
    mbc: Box<dyn MemoryBankController>,
}

impl MemoryMappedHardware for GamePak {
    fn read_value(&self, address: u16) -> u8 {
        self.mbc.read_value(address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        self.mbc.set_value(address, value);
    }
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
            .unwrap_or(title_slice.len());
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

        let mbc: Box<dyn MemoryBankController> = match rom[MBC_TYPE_ADDRESS] {
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

    pub fn tick(&mut self) {
        self.mbc.tick();
    }

    #[cfg(test)]
    pub fn title(&self) -> &str {
        &self.title
    }
}

impl fmt::Debug for GamePak {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GamePak({:?}, {:?})", self.title, self.mbc)
    }
}
