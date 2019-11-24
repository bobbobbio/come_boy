// Copyright 2019 Remi Bernotavicius

use super::memory_controller::{MemoryChunk, MemoryMappedHardware};
use crate::util::super_fast_hash;
use std::fmt;
use std::io::{self, Read};
use std::ops::Range;
use std::str;

trait MemoryBankController: fmt::Debug + MemoryMappedHardware {
    fn tick(&mut self) {}

    // XXX Weird workaround to implement Clone for Box<dyn MemoryBankController>
    fn box_clone(&self) -> Box<dyn MemoryBankController>;
}

impl Clone for Box<dyn MemoryBankController> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

#[derive(Clone)]
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
    fn box_clone(&self) -> Box<dyn MemoryBankController> {
        Box::new((*self).clone())
    }
}

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

#[derive(Clone)]
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
struct MemoryBankController1<R: CartridgeRam> {
    rom_bank_number: usize,
    ram_bank_number: usize,
    rom_ram_select: bool,
    ram_enable: bool,
    switchable_bank: SwitchableBank,
    ram: R,
}

impl<R: CartridgeRam> MemoryMappedHardware for MemoryBankController1<R> {
    fn read_value(&self, address: u16) -> u8 {
        if address < 0x4000 {
            self.switchable_bank.banks[0].read_value(address)
        } else if address < 0xA000 {
            self.switchable_bank.read_value(address - 0x4000)
        } else if self.ram_enable {
            self.ram.read_value(address - 0xA000)
        } else {
            0xFF
        }
    }

    fn set_value(&mut self, address: u16, value: u8) {
        if address < 0x2000 {
            self.ram_enable = (value & 0x0F) == 0x0A;
        } else if address < 0x4000 {
            self.rom_bank_number = (value as usize) & 0x1F;
        } else if address < 0x6000 {
            if self.rom_ram_select {
                self.ram_bank_number = value as usize;
            } else {
                self.rom_bank_number |= ((value as usize) & 0x03) << 5;
            }
        } else if address < 0x8000 {
            self.rom_ram_select = value > 0;
        } else if address < 0xA000 {
            // nothing?
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
        MemoryBankController1 {
            rom_bank_number: 0,
            ram_bank_number: 0,
            rom_ram_select: false,
            ram_enable: false,
            switchable_bank: SwitchableBank::new(banks),
            ram,
        }
    }
}

impl<R: CartridgeRam + 'static> MemoryBankController for MemoryBankController1<R> {
    fn tick(&mut self) {
        let mut rom_bank_value = (self.rom_bank_number as u8 & 0x0F) as usize;
        rom_bank_value %= self.switchable_bank.len();

        self.switchable_bank.switch_bank(rom_bank_value);

        if self.ram_enable {
            self.ram.switch_bank(self.ram_bank_number);
        }
    }

    fn box_clone(&self) -> Box<dyn MemoryBankController> {
        Box::new((*self).clone())
    }
}

#[derive(Clone)]
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
        if address >= 0x3000 && address < 0x4000 {
            self.inner.rom_bank_number |= ((value & 0x01) as usize) << 8;
        } else {
            self.inner.set_value(address, value);
        }
    }
}

impl<R: CartridgeRam + 'static> MemoryBankController for MemoryBankController5<R> {
    fn tick(&mut self) {
        self.inner.tick();
    }

    fn box_clone(&self) -> Box<dyn MemoryBankController> {
        Box::new((*self).clone())
    }
}

trait CartridgeRam: fmt::Debug + MemoryMappedHardware + Clone {
    fn switch_bank(&mut self, bank: usize);
}

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct GamePak {
    title: String,
    hash: u32,
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
    pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> io::Result<Self> {
        let mut rom_file = std::fs::File::open(path)?;
        let mut rom: Vec<u8> = vec![];
        rom_file.read_to_end(&mut rom)?;
        Ok(GamePak::new(&rom))
    }

    pub fn new(rom: &[u8]) -> Self {
        assert_eq!(rom.len() % (BANK_SIZE as usize), 0, "ROM wrong size");
        let hash = super_fast_hash(rom);
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

        GamePak { title, hash, mbc }
    }

    pub fn tick(&mut self) {
        self.mbc.tick();
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn hash(&self) -> u32 {
        self.hash
    }
}

impl fmt::Debug for GamePak {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GamePak({:?}, {:?})", self.title, self.mbc)
    }
}
