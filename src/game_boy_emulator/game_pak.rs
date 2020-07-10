// Copyright 2019 Remi Bernotavicius

use super::memory_controller::{MemoryChunk, MemoryMappedHardware};
use crate::util::super_fast_hash;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::str;

#[derive(Clone, Serialize, Deserialize)]
struct RomBank(#[serde(skip)] MemoryChunk);

impl RomBank {
    fn new(value: Vec<u8>) -> Self {
        Self(MemoryChunk::new(value))
    }
}

impl MemoryMappedHardware for RomBank {
    fn read_value(&self, address: u16) -> u8 {
        self.0.read_value(address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        self.0.set_value(address, value)
    }
}

#[derive(Serialize, Deserialize)]
enum MemoryBankController {
    Zero(MemoryBankController0),
    One(MemoryBankController1),
    Two(MemoryBankController2),
    Three(MemoryBankController3),
    Five(MemoryBankController5),
}

impl fmt::Debug for MemoryBankController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Zero(r) => write!(f, "{:?}", r),
            Self::One(r) => write!(f, "{:?}", r),
            Self::Two(r) => write!(f, "{:?}", r),
            Self::Three(r) => write!(f, "{:?}", r),
            Self::Five(r) => write!(f, "{:?}", r),
        }
    }
}

impl MemoryMappedHardware for MemoryBankController {
    fn read_value(&self, address: u16) -> u8 {
        match self {
            Self::Zero(r) => r.read_value(address),
            Self::One(r) => r.read_value(address),
            Self::Two(r) => r.read_value(address),
            Self::Three(r) => r.read_value(address),
            Self::Five(r) => r.read_value(address),
        }
    }

    fn set_value(&mut self, address: u16, value: u8) {
        match self {
            Self::Zero(r) => r.set_value(address, value),
            Self::One(r) => r.set_value(address, value),
            Self::Two(r) => r.set_value(address, value),
            Self::Three(r) => r.set_value(address, value),
            Self::Five(r) => r.set_value(address, value),
        }
    }
}

impl MemoryBankController {
    fn tick(&mut self) {
        match self {
            Self::Zero(r) => r.tick(),
            Self::One(r) => r.tick(),
            Self::Two(r) => r.tick(),
            Self::Three(r) => r.tick(),
            Self::Five(r) => r.tick(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct MemoryBankController0 {
    banks: Vec<RomBank>,
}

impl Into<MemoryBankController> for MemoryBankController0 {
    fn into(self) -> MemoryBankController {
        MemoryBankController::Zero(self)
    }
}

impl MemoryBankController0 {
    fn new(banks: Vec<RomBank>) -> Self {
        MemoryBankController0 { banks }
    }

    fn tick(&mut self) {}
}

impl fmt::Debug for MemoryBankController0 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MBC0")
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

#[derive(Clone, Serialize, Deserialize)]
struct SwitchableBank<T> {
    banks: Vec<T>,
    current_bank: usize,
}

impl<T> SwitchableBank<T> {
    fn new(banks: Vec<T>) -> Self {
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

impl SwitchableBank<MemoryChunk> {
    fn total_len(&self) -> usize {
        let mut len = 0;
        for b in &self.banks {
            len += b.len() as usize;
        }
        len
    }

    fn current_bank_offset(&self) -> usize {
        let mut offset = 0;
        for b in &self.banks[..self.current_bank] {
            offset += b.len() as usize;
        }
        offset
    }
}

impl<T: MemoryMappedHardware> MemoryMappedHardware for SwitchableBank<T> {
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

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
enum RomOrRam {
    Rom,
    Ram,
}

#[derive(Serialize, Deserialize)]
struct MemoryBankController1 {
    rom_bank_number: usize,
    ram_bank_number: usize,
    rom_ram_select: RomOrRam,
    ram_enable: bool,
    switchable_bank: SwitchableBank<RomBank>,
    ram: CartridgeRam,
}

impl Into<MemoryBankController> for MemoryBankController1 {
    fn into(self) -> MemoryBankController {
        MemoryBankController::One(self)
    }
}

impl MemoryMappedHardware for MemoryBankController1 {
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
            // Enable RAM
            self.ram_enable = (value & 0x0F) == 0x0A;
        } else if address < 0x4000 {
            // Select ROM bank (lower 5 bits)
            self.rom_bank_number &= !0x1F;
            self.rom_bank_number |= (value as usize) & 0x1F;
            if self.rom_bank_number == 0 {
                self.rom_bank_number = 1;
            }
        } else if address < 0x6000 {
            // Either select RAM bank or select ROM bank (6th and 7th bit)
            if self.rom_ram_select == RomOrRam::Ram {
                self.ram_bank_number = value as usize;
            } else {
                self.rom_bank_number &= !(0x03 << 5);
                self.rom_bank_number |= ((value as usize) & 0x03) << 5;
            }
        } else if address < 0x8000 {
            // Either select RAM bank or upper bits of ROM bank
            if value > 0 {
                self.rom_ram_select = RomOrRam::Ram;
            } else {
                self.rom_ram_select = RomOrRam::Rom;
            }
        } else if address < 0xA000 {
            // nothing
        } else {
            self.ram.set_value(address - 0xA000, value);
        }
    }
}

impl fmt::Debug for MemoryBankController1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MBC1{:?}", self.ram)
    }
}

impl MemoryBankController1 {
    fn new<R: Into<CartridgeRam>>(banks: Vec<RomBank>, ram: R) -> Self {
        MemoryBankController1 {
            rom_bank_number: 0,
            ram_bank_number: 0,
            rom_ram_select: RomOrRam::Rom,
            ram_enable: false,
            switchable_bank: SwitchableBank::new(banks),
            ram: ram.into(),
        }
    }

    fn tick(&mut self) {
        let rom_bank_value = self.rom_bank_number % self.switchable_bank.len();

        self.switchable_bank.switch_bank(rom_bank_value);

        if self.ram_enable {
            self.ram.switch_bank(self.ram_bank_number);
        }
    }
}

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
enum ClockOrRam {
    Clock,
    Ram,
}

#[derive(Serialize, Deserialize)]
struct MemoryBankController3 {
    inner: MemoryBankController1,
    clock_ram_select: ClockOrRam,
}

impl Into<MemoryBankController> for MemoryBankController3 {
    fn into(self) -> MemoryBankController {
        MemoryBankController::Three(self)
    }
}

impl fmt::Debug for MemoryBankController3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MBC3{:?}", self.inner.ram)
    }
}

impl MemoryBankController3 {
    fn new<R: Into<CartridgeRam>>(banks: Vec<RomBank>, ram: R) -> Self {
        MemoryBankController3 {
            inner: MemoryBankController1::new(banks, ram),
            clock_ram_select: ClockOrRam::Ram,
        }
    }

    fn tick(&mut self) {
        self.inner.tick();
    }
}

impl MemoryMappedHardware for MemoryBankController3 {
    fn read_value(&self, address: u16) -> u8 {
        if address < 0xA000 {
            self.inner.read_value(address)
        } else if address < 0xC000 {
            if self.clock_ram_select == ClockOrRam::Clock {
                // Read current RTC register
                0xFF
            } else {
                self.inner.ram.read_value(address - 0xA000)
            }
        } else {
            self.inner.read_value(address)
        }
    }

    fn set_value(&mut self, address: u16, value: u8) {
        if address < 0x2000 {
            self.inner.set_value(address, value);
        } else if address < 0x4000 {
            // Select ROM bank (lower 7 bits)
            self.inner.rom_bank_number &= !0x7F;
            self.inner.rom_bank_number |= (value as usize) & 0x7F;
            if self.inner.rom_bank_number == 0 {
                self.inner.rom_bank_number = 1;
            }
        } else if address < 0x6000 {
            if value < 0x08 {
                self.clock_ram_select = ClockOrRam::Ram;
                self.inner.ram_bank_number = value as usize;
            } else {
                self.clock_ram_select = ClockOrRam::Clock;
                // Select RTC register
            }
        } else if address < 0x8000 {
            // Latch clock data
        } else if address < 0xA000 {
            self.inner.set_value(address, value);
        } else if address < 0xC000 {
            if self.clock_ram_select == ClockOrRam::Clock {
                // Write to RTC register
            } else {
                self.inner.ram.set_value(address - 0xA000, value);
            }
        } else {
            self.inner.set_value(address, value);
        }
    }
}

#[derive(Serialize, Deserialize)]
enum InternalRam {
    Volatile(VolatileInternalRam),
    NonVolatile(NonVolatileInternalRam),
}

impl MemoryMappedHardware for InternalRam {
    fn read_value(&self, address: u16) -> u8 {
        match self {
            Self::Volatile(r) => r.read_value(address),
            Self::NonVolatile(r) => r.read_value(address),
        }
    }

    fn set_value(&mut self, address: u16, value: u8) {
        match self {
            Self::Volatile(r) => r.set_value(address, value),
            Self::NonVolatile(r) => r.set_value(address, value),
        }
    }
}

impl fmt::Debug for InternalRam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Volatile(r) => write!(f, "{:?}", r),
            Self::NonVolatile(r) => write!(f, "{:?}", r),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct VolatileInternalRam(MemoryChunk);

impl Into<InternalRam> for VolatileInternalRam {
    fn into(self) -> InternalRam {
        InternalRam::Volatile(self)
    }
}

impl VolatileInternalRam {
    fn new() -> Self {
        Self(MemoryChunk::from_range(0..512))
    }
}

impl MemoryMappedHardware for VolatileInternalRam {
    fn read_value(&self, address: u16) -> u8 {
        self.0.read_value(address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        self.0.set_value(address, value & 0x0F);
    }
}

impl fmt::Debug for VolatileInternalRam {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct NonVolatileInternalRam {
    memory: MemoryChunk,
    #[serde(skip)]
    file: Option<File>,
}

impl Into<InternalRam> for NonVolatileInternalRam {
    fn into(self) -> InternalRam {
        InternalRam::NonVolatile(self)
    }
}

impl NonVolatileInternalRam {
    fn new(path: Option<PathBuf>) -> Self {
        let mut file = if let Some(path) = path {
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path)
                .ok()
        } else {
            None
        };

        let memory = if let Some(file) = &mut file {
            MemoryChunk::from_reader(file, 512).unwrap_or(MemoryChunk::from_range(0..512))
        } else {
            MemoryChunk::from_range(0..512)
        };

        if let Some(file) = &mut file {
            file.set_len(512).ok();
        }

        Self { memory, file }
    }
}

impl MemoryMappedHardware for NonVolatileInternalRam {
    fn read_value(&self, address: u16) -> u8 {
        self.memory.read_value(address) | 0xF0
    }
    fn set_value(&mut self, address: u16, value: u8) {
        if address >= self.memory.len() {
            return;
        }

        self.memory.set_value(address, value | 0xF0);
        if let Some(file) = &mut self.file {
            file.seek(SeekFrom::Start(address as u64)).ok();
            file.write(&[value | 0xF0]).ok();
        }
    }
}

impl fmt::Debug for NonVolatileInternalRam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+BATTERY")
    }
}

#[derive(Serialize, Deserialize)]
struct MemoryBankController2 {
    internal_ram: InternalRam,
    rom_bank_number: usize,
    ram_enable: bool,
    switchable_bank: SwitchableBank<RomBank>,
}

impl Into<MemoryBankController> for MemoryBankController2 {
    fn into(self) -> MemoryBankController {
        MemoryBankController::Two(self)
    }
}

impl fmt::Debug for MemoryBankController2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MBC2{:?}", self.internal_ram)
    }
}

impl MemoryBankController2 {
    fn new<R: Into<InternalRam>>(banks: Vec<RomBank>, ram: R) -> Self {
        MemoryBankController2 {
            internal_ram: ram.into(),
            rom_bank_number: 0,
            ram_enable: false,
            switchable_bank: SwitchableBank::new(banks),
        }
    }

    fn tick(&mut self) {
        let rom_bank_value = self.rom_bank_number % self.switchable_bank.len();
        self.switchable_bank.switch_bank(rom_bank_value);
    }
}

impl MemoryMappedHardware for MemoryBankController2 {
    fn read_value(&self, address: u16) -> u8 {
        if address < 0x4000 {
            self.switchable_bank.banks[0].read_value(address)
        } else if address < 0xA000 {
            self.switchable_bank.read_value(address - 0x4000)
        } else if self.ram_enable {
            self.internal_ram.read_value(address - 0xA000)
        } else {
            0xFF
        }
    }

    fn set_value(&mut self, address: u16, value: u8) {
        if address < 0x2000 {
            if (address >> 8) & 0x01 == 0 {
                self.ram_enable = (value & 0x0F) == 0x0A;
            }
        } else if address < 0x4000 {
            if (address >> 8) & 0x01 == 1 {
                self.rom_bank_number = (value & 0x0F) as usize;
            }
        } else if address >= 0xA000 && address < 0xA200 {
            if self.ram_enable {
                self.internal_ram.set_value(address - 0xA000, value);
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct MemoryBankController5 {
    inner: MemoryBankController1,
}

impl Into<MemoryBankController> for MemoryBankController5 {
    fn into(self) -> MemoryBankController {
        MemoryBankController::Five(self)
    }
}

impl fmt::Debug for MemoryBankController5 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MBC5{:?}", self.inner.ram)
    }
}

impl MemoryBankController5 {
    fn new<R: Into<CartridgeRam>>(banks: Vec<RomBank>, ram: R) -> Self {
        MemoryBankController5 {
            inner: MemoryBankController1::new(banks, ram),
        }
    }

    fn tick(&mut self) {
        self.inner.tick();
    }
}

impl MemoryMappedHardware for MemoryBankController5 {
    fn read_value(&self, address: u16) -> u8 {
        self.inner.read_value(address)
    }

    fn set_value(&mut self, address: u16, value: u8) {
        if address < 0x2000 {
            // RAM Enable
            self.inner.set_value(address, value);
        } else if address < 0x3000 {
            // Select ROM bank (lower 8 bits)
            self.inner.rom_bank_number &= !0xFF;
            self.inner.rom_bank_number |= value as usize;
        } else if address < 0x4000 {
            // Select ROM bank (9th bit)
            self.inner.rom_bank_number &= !(0x01 << 8);
            self.inner.rom_bank_number |= ((value & 0x01) as usize) << 8;
        } else if address < 0x6000 {
            // Select RAM bank
            self.inner.ram_bank_number = value as usize;
        } else if address < 0xA000 {
            // nothing
        } else {
            // Write to RAM
            self.inner.set_value(address, value);
        }
    }
}

#[derive(Serialize, Deserialize)]
enum CartridgeRam {
    No(NoRam),
    Volatile(VolatileRam),
    NonVolatile(NonVolatileRam),
}

impl fmt::Debug for CartridgeRam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::No(r) => write!(f, "{:?}", r),
            Self::Volatile(r) => write!(f, "{:?}", r),
            Self::NonVolatile(r) => write!(f, "{:?}", r),
        }
    }
}

impl MemoryMappedHardware for CartridgeRam {
    fn read_value(&self, address: u16) -> u8 {
        match self {
            Self::No(r) => r.read_value(address),
            Self::Volatile(r) => r.read_value(address),
            Self::NonVolatile(r) => r.read_value(address),
        }
    }

    fn set_value(&mut self, address: u16, value: u8) {
        match self {
            Self::No(r) => r.set_value(address, value),
            Self::Volatile(r) => r.set_value(address, value),
            Self::NonVolatile(r) => r.set_value(address, value),
        }
    }
}

impl CartridgeRam {
    fn switch_bank(&mut self, bank: usize) {
        match self {
            Self::Volatile(r) => r.switch_bank(bank),
            _ => (),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct NoRam;

impl Into<CartridgeRam> for NoRam {
    fn into(self) -> CartridgeRam {
        CartridgeRam::No(self)
    }
}

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

#[derive(Clone, Serialize, Deserialize)]
struct VolatileRam {
    switchable_bank: SwitchableBank<MemoryChunk>,
}

impl Into<CartridgeRam> for VolatileRam {
    fn into(self) -> CartridgeRam {
        CartridgeRam::Volatile(self)
    }
}

impl fmt::Debug for VolatileRam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+RAM")
    }
}

impl VolatileRam {
    fn new(ram_size: u8) -> Self {
        let ram = match ram_size {
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
        VolatileRam {
            switchable_bank: SwitchableBank::new(ram),
        }
    }

    fn switch_bank(&mut self, bank: usize) {
        self.switchable_bank.switch_bank(bank)
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

#[derive(Serialize, Deserialize)]
struct NonVolatileRam {
    switchable_bank: SwitchableBank<MemoryChunk>,
    #[serde(skip)]
    file: Option<File>,
}

impl Into<CartridgeRam> for NonVolatileRam {
    fn into(self) -> CartridgeRam {
        CartridgeRam::NonVolatile(self)
    }
}

impl fmt::Debug for NonVolatileRam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+RAM+BATTERY")
    }
}

impl NonVolatileRam {
    fn new(ram_size: u8, path: Option<PathBuf>) -> Self {
        let mut file = if let Some(path) = path {
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path)
                .ok()
        } else {
            None
        };

        let mut file_size = 0;

        let mut chunk_factory = |size: u16| {
            if let Some(file) = &mut file {
                file_size += size;
                MemoryChunk::from_reader(file, size as usize)
                    .unwrap_or(MemoryChunk::from_range(0..size))
            } else {
                MemoryChunk::from_range(0..size)
            }
        };

        let ram = match ram_size {
            0 => Vec::new(),
            // 2kB 1 Bank
            1 => vec![chunk_factory(0x800)],
            // 8kB 1 Bank
            2 => vec![chunk_factory(0x2000)],
            // 8kB 4 Banks = 32kB
            3 => vec![
                chunk_factory(0x2000),
                chunk_factory(0x2000),
                chunk_factory(0x2000),
                chunk_factory(0x2000),
            ],
            v => panic!("Unknown RAM size {}", v),
        };

        if let Some(file) = &mut file {
            file.set_len(file_size as u64).ok();
        }

        NonVolatileRam {
            switchable_bank: SwitchableBank::new(ram),
            file,
        }
    }
}

impl MemoryMappedHardware for NonVolatileRam {
    fn read_value(&self, address: u16) -> u8 {
        self.switchable_bank.read_value(address)
    }
    fn set_value(&mut self, address: u16, value: u8) {
        if address as usize >= self.switchable_bank.total_len() {
            return;
        }

        self.switchable_bank.set_value(address, value);
        if let Some(file) = &mut self.file {
            file.seek(SeekFrom::Start(
                (self.switchable_bank.current_bank_offset() + address as usize) as u64,
            ))
            .ok();
            file.write(&[value]).ok();
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GamePak {
    title: String,
    hash: u32,
    mbc: MemoryBankController,
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
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path: &Path = path.as_ref();
        let mut rom_file = std::fs::File::open(path)?;
        let mut rom: Vec<u8> = vec![];
        rom_file.read_to_end(&mut rom)?;
        Ok(GamePak::new(&rom, Some(path.with_extension("sav"))))
    }

    pub fn new(rom: &[u8], sram_path: Option<PathBuf>) -> Self {
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
            banks.push(RomBank::new(rom[start..end].to_vec()));
        }

        let ram_size = rom[RAM_SIZE_ADDRESS];

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

        let mbc: MemoryBankController =
            match rom[MBC_TYPE_ADDRESS] {
                0x00 => {
                    assert_eq!(number_of_banks, 2);
                    MemoryBankController0::new(banks).into()
                }
                0x01 => MemoryBankController1::new(banks, NoRam).into(),
                0x02 => MemoryBankController1::new(banks, VolatileRam::new(ram_size)).into(),
                0x03 => MemoryBankController1::new(banks, NonVolatileRam::new(ram_size, sram_path))
                    .into(),
                0x05 => MemoryBankController2::new(banks, VolatileInternalRam::new()).into(),
                0x06 => {
                    MemoryBankController2::new(banks, NonVolatileInternalRam::new(sram_path)).into()
                }
                0x11 => MemoryBankController3::new(banks, NoRam).into(),
                0x12 => MemoryBankController3::new(banks, VolatileRam::new(ram_size)).into(),
                0x13 => MemoryBankController3::new(banks, NonVolatileRam::new(ram_size, sram_path))
                    .into(),
                0x1b => MemoryBankController5::new(banks, NonVolatileRam::new(ram_size, sram_path))
                    .into(),
                v => panic!("Unknown Memory Bank Controller {:#x}", v),
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
