// Copyright 2019 Remi Bernotavicius

use super::memory_controller::{MemoryChunk, MemoryMappedHardware};
use crate::io::{self, Read as _, Seek as _, SeekFrom, Write as _};
use crate::storage::{OpenMode, PersistentStorage, StorageFile as _};
use alloc::{format, string::String, vec, vec::Vec};
use core::fmt;
use core::ops::Range;
use core::str;
use serde_derive::{Deserialize, Serialize};

pub fn rom_hash(rom: &[u8]) -> u32 {
    crate::util::super_fast_hash(rom)
}

struct BankOps<Storage: PersistentStorage> {
    sram_file: Option<Storage::File>,
    rom_memory: Vec<MemoryChunk>,
}

trait MemoryMappedBank {
    fn read_bank_value(&self, ops: &BankOps<impl PersistentStorage>, address: u16) -> u8;
    fn set_bank_value(
        &mut self,
        ops: &mut BankOps<impl PersistentStorage>,
        address: u16,
        value: u8,
    );
}

#[derive(Clone, Serialize, Deserialize)]
struct RomBank(usize);

impl RomBank {
    fn new(index: usize) -> Self {
        Self(index)
    }
}

impl MemoryMappedBank for RomBank {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_bank_value(&self, ops: &BankOps<impl PersistentStorage>, address: u16) -> u8 {
        ops.rom_memory[self.0].read_value(address)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_bank_value(
        &mut self,
        ops: &mut BankOps<impl PersistentStorage>,
        address: u16,
        value: u8,
    ) {
        ops.rom_memory[self.0].set_value(address, value)
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
            Self::Zero(r) => write!(f, "{r:?}"),
            Self::One(r) => write!(f, "{r:?}"),
            Self::Two(r) => write!(f, "{r:?}"),
            Self::Three(r) => write!(f, "{r:?}"),
            Self::Five(r) => write!(f, "{r:?}"),
        }
    }
}

impl MemoryMappedBank for MemoryBankController {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_bank_value(&self, ops: &BankOps<impl PersistentStorage>, address: u16) -> u8 {
        match self {
            Self::Zero(r) => r.read_bank_value(ops, address),
            Self::One(r) => r.read_bank_value(ops, address),
            Self::Two(r) => r.read_bank_value(ops, address),
            Self::Three(r) => r.read_bank_value(ops, address),
            Self::Five(r) => r.read_bank_value(ops, address),
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_bank_value(
        &mut self,
        ops: &mut BankOps<impl PersistentStorage>,
        address: u16,
        value: u8,
    ) {
        match self {
            Self::Zero(r) => r.set_bank_value(ops, address, value),
            Self::One(r) => r.set_bank_value(ops, address, value),
            Self::Two(r) => r.set_bank_value(ops, address, value),
            Self::Three(r) => r.set_bank_value(ops, address, value),
            Self::Five(r) => r.set_bank_value(ops, address, value),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct MemoryBankController0 {
    banks: Vec<RomBank>,
}

impl From<MemoryBankController0> for MemoryBankController {
    fn from(c: MemoryBankController0) -> Self {
        Self::Zero(c)
    }
}

impl MemoryBankController0 {
    fn new(banks: Vec<RomBank>) -> Self {
        assert_eq!(banks.len(), 2);
        MemoryBankController0 { banks }
    }
}

impl fmt::Debug for MemoryBankController0 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MBC0")
    }
}

impl MemoryMappedBank for MemoryBankController0 {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_bank_value(&self, ops: &BankOps<impl PersistentStorage>, address: u16) -> u8 {
        if address < 0x4000 {
            self.banks[0].read_bank_value(ops, address)
        } else if address < 0x8000 {
            self.banks[1].read_bank_value(ops, address - 0x4000)
        } else {
            0xFF
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_bank_value(
        &mut self,
        _ops: &mut BankOps<impl PersistentStorage>,
        _address: u16,
        _value: u8,
    ) {
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
struct SwitchableBank<T> {
    banks: Vec<T>,
    current_bank: usize,
}

impl<T> SwitchableBank<T> {
    fn new(banks: Vec<T>, current_bank: usize) -> Self {
        SwitchableBank {
            banks,
            current_bank,
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn switch_bank(&mut self, new_bank: usize) {
        assert!(new_bank < self.banks.len());
        self.current_bank = new_bank;
    }
}

impl SwitchableBank<SramChunk> {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn total_len(&self) -> usize {
        let mut len = 0;
        for b in &self.banks {
            len += b.len() as usize;
        }
        len
    }
}

impl<T: MemoryMappedBank> MemoryMappedBank for SwitchableBank<T> {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_bank_value(&self, ops: &BankOps<impl PersistentStorage>, address: u16) -> u8 {
        if self.current_bank >= self.banks.len() {
            0xFF
        } else {
            self.banks[self.current_bank].read_bank_value(ops, address)
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_bank_value(
        &mut self,
        ops: &mut BankOps<impl PersistentStorage>,
        address: u16,
        value: u8,
    ) {
        if self.current_bank < self.banks.len() {
            self.banks[self.current_bank].set_bank_value(ops, address, value);
        }
    }
}

impl<T: MemoryMappedHardware> MemoryMappedHardware for SwitchableBank<T> {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_value(&self, address: u16) -> u8 {
        if self.current_bank >= self.banks.len() {
            0xFF
        } else {
            self.banks[self.current_bank].read_value(address)
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
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

impl From<MemoryBankController1> for MemoryBankController {
    fn from(c: MemoryBankController1) -> Self {
        Self::One(c)
    }
}

impl MemoryMappedBank for MemoryBankController1 {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_bank_value(&self, ops: &BankOps<impl PersistentStorage>, address: u16) -> u8 {
        if address < 0x4000 {
            self.switchable_bank.banks[0].read_bank_value(ops, address)
        } else if address < 0xA000 {
            self.switchable_bank.read_bank_value(ops, address - 0x4000)
        } else if self.ram_enable {
            self.ram.read_bank_value(ops, address - 0xA000)
        } else {
            0xFF
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_bank_value(
        &mut self,
        ops: &mut BankOps<impl PersistentStorage>,
        address: u16,
        value: u8,
    ) {
        if address < 0x2000 {
            // Enable RAM
            self.ram_enable = (value & 0x0F) == 0x0A;
        } else if address < 0x4000 {
            // Select ROM bank (lower 5 bits)
            self.rom_bank_number &= !0x1F;
            self.rom_bank_number |= (value as usize) & 0x1F;
            self.update_rom_bank();
        } else if address < 0x6000 {
            // Either select RAM bank or select ROM bank (6th and 7th bit)
            if self.rom_ram_select == RomOrRam::Ram {
                self.ram_bank_number = value as usize;
                self.update_ram_bank();
            } else {
                self.rom_bank_number &= !(0x03 << 5);
                self.rom_bank_number |= ((value as usize) & 0x03) << 5;
                self.update_rom_bank();
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
            self.ram.set_bank_value(ops, address - 0xA000, value);
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
            rom_bank_number: 1,
            ram_bank_number: 0,
            rom_ram_select: RomOrRam::Rom,
            ram_enable: false,
            switchable_bank: SwitchableBank::new(banks, 1),
            ram: ram.into(),
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn update_rom_bank(&mut self) {
        if self.rom_bank_number == 0 {
            self.rom_bank_number = 1;
        }
        self.switchable_bank.switch_bank(self.rom_bank_number);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn update_ram_bank(&mut self) {
        self.ram.switch_bank(self.ram_bank_number);
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

impl From<MemoryBankController3> for MemoryBankController {
    fn from(c: MemoryBankController3) -> Self {
        Self::Three(c)
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
}

impl MemoryMappedBank for MemoryBankController3 {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_bank_value(&self, ops: &BankOps<impl PersistentStorage>, address: u16) -> u8 {
        if address < 0xA000 {
            self.inner.read_bank_value(ops, address)
        } else if address < 0xC000 {
            if self.clock_ram_select == ClockOrRam::Clock {
                // Read current RTC register
                0xFF
            } else {
                self.inner.ram.read_bank_value(ops, address - 0xA000)
            }
        } else {
            self.inner.read_bank_value(ops, address)
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_bank_value(
        &mut self,
        ops: &mut BankOps<impl PersistentStorage>,
        address: u16,
        value: u8,
    ) {
        if address < 0x2000 {
            self.inner.set_bank_value(ops, address, value);
        } else if address < 0x4000 {
            // Select ROM bank (lower 7 bits)
            self.inner.rom_bank_number &= !0x7F;
            self.inner.rom_bank_number |= (value as usize) & 0x7F;
            self.inner.update_rom_bank();
        } else if address < 0x6000 {
            if value < 0x08 {
                self.clock_ram_select = ClockOrRam::Ram;
                self.inner.ram_bank_number = value as usize;
                self.inner.update_ram_bank();
            } else {
                self.clock_ram_select = ClockOrRam::Clock;
                // Select RTC register
            }
        } else if address < 0x8000 {
            // Latch clock data
        } else if address < 0xA000 {
            self.inner.set_bank_value(ops, address, value);
        } else if address < 0xC000 {
            if self.clock_ram_select == ClockOrRam::Clock {
                // Write to RTC register
            } else {
                self.inner.ram.set_bank_value(ops, address - 0xA000, value);
            }
        } else {
            self.inner.set_bank_value(ops, address, value);
        }
    }
}

#[derive(Serialize, Deserialize)]
enum InternalRam {
    Volatile(VolatileInternalRam),
    NonVolatile(NonVolatileInternalRam),
}

impl MemoryMappedBank for InternalRam {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_bank_value(&self, ops: &BankOps<impl PersistentStorage>, address: u16) -> u8 {
        match self {
            Self::Volatile(r) => r.read_value(address),
            Self::NonVolatile(r) => r.read_bank_value(ops, address),
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_bank_value(
        &mut self,
        ops: &mut BankOps<impl PersistentStorage>,
        address: u16,
        value: u8,
    ) {
        match self {
            Self::Volatile(r) => r.set_value(address, value),
            Self::NonVolatile(r) => r.set_bank_value(ops, address, value),
        }
    }
}

impl fmt::Debug for InternalRam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Volatile(r) => write!(f, "{r:?}"),
            Self::NonVolatile(r) => write!(f, "{r:?}"),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct VolatileInternalRam(MemoryChunk);

impl From<VolatileInternalRam> for InternalRam {
    fn from(c: VolatileInternalRam) -> Self {
        Self::Volatile(c)
    }
}

impl VolatileInternalRam {
    fn new() -> Self {
        Self(MemoryChunk::from_range(0..512))
    }
}

impl MemoryMappedHardware for VolatileInternalRam {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_value(&self, address: u16) -> u8 {
        self.0.read_value(address)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
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
struct NonVolatileInternalRam(SramChunk);

impl From<NonVolatileInternalRam> for InternalRam {
    fn from(c: NonVolatileInternalRam) -> Self {
        Self::NonVolatile(c)
    }
}

impl NonVolatileInternalRam {
    fn new(sram: Vec<SramChunk>) -> Self {
        assert_eq!(sram.len(), 1);
        Self(sram.into_iter().next().unwrap())
    }
}

impl MemoryMappedBank for NonVolatileInternalRam {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_bank_value(&self, ops: &BankOps<impl PersistentStorage>, address: u16) -> u8 {
        self.0.read_bank_value(ops, address) | 0xF0
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_bank_value(
        &mut self,
        ops: &mut BankOps<impl PersistentStorage>,
        address: u16,
        value: u8,
    ) {
        if address >= self.0.len() {
            return;
        }

        self.0.set_bank_value(ops, address, value | 0xF0);
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

impl From<MemoryBankController2> for MemoryBankController {
    fn from(c: MemoryBankController2) -> Self {
        Self::Two(c)
    }
}

impl fmt::Debug for MemoryBankController2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MBC2{:?}", self.internal_ram)
    }
}

impl MemoryBankController2 {
    fn new<R: Into<InternalRam>>(banks: Vec<RomBank>, ram: R) -> Self {
        Self {
            internal_ram: ram.into(),
            rom_bank_number: 1,
            ram_enable: false,
            switchable_bank: SwitchableBank::new(banks, 1),
        }
    }

    fn update_rom_bank(&mut self) {
        if self.rom_bank_number == 0 {
            self.rom_bank_number = 1;
        }

        self.switchable_bank.switch_bank(self.rom_bank_number);
    }
}

impl MemoryMappedBank for MemoryBankController2 {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_bank_value(&self, ops: &BankOps<impl PersistentStorage>, address: u16) -> u8 {
        if address < 0x4000 {
            self.switchable_bank.banks[0].read_bank_value(ops, address)
        } else if address < 0xA000 {
            self.switchable_bank.read_bank_value(ops, address - 0x4000)
        } else if self.ram_enable {
            self.internal_ram.read_bank_value(ops, address - 0xA000)
        } else {
            0xFF
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_bank_value(
        &mut self,
        ops: &mut BankOps<impl PersistentStorage>,
        address: u16,
        value: u8,
    ) {
        if address < 0x2000 {
            if (address >> 8) & 0x01 == 0 {
                self.ram_enable = (value & 0x0F) == 0x0A;
            }
        } else if address < 0x4000 {
            if (address >> 8) & 0x01 == 1 {
                self.rom_bank_number = (value & 0x0F) as usize;
                self.update_rom_bank();
            }
        } else if (0xA000..0xA200).contains(&address) && self.ram_enable {
            self.internal_ram
                .set_bank_value(ops, address - 0xA000, value);
        }
    }
}

#[derive(Serialize, Deserialize)]
struct MemoryBankController5 {
    inner: MemoryBankController1,
}

impl From<MemoryBankController5> for MemoryBankController {
    fn from(c: MemoryBankController5) -> Self {
        Self::Five(c)
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
}

impl MemoryMappedBank for MemoryBankController5 {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_bank_value(&self, ops: &BankOps<impl PersistentStorage>, address: u16) -> u8 {
        self.inner.read_bank_value(ops, address)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_bank_value(
        &mut self,
        ops: &mut BankOps<impl PersistentStorage>,
        address: u16,
        value: u8,
    ) {
        if address < 0x2000 {
            // RAM Enable
            self.inner.set_bank_value(ops, address, value);
        } else if address < 0x3000 {
            // Select ROM bank (lower 8 bits)
            self.inner.rom_bank_number &= !0xFF;
            self.inner.rom_bank_number |= value as usize;
            self.inner.update_rom_bank();
        } else if address < 0x4000 {
            // Select ROM bank (9th bit)
            self.inner.rom_bank_number &= !(0x01 << 8);
            self.inner.rom_bank_number |= ((value & 0x01) as usize) << 8;
            self.inner.update_rom_bank();
        } else if address < 0x6000 {
            // Select RAM bank
            self.inner.ram_bank_number = value as usize;
            self.inner.update_ram_bank();
        } else if address < 0xA000 {
            // nothing
        } else {
            // Write to RAM
            self.inner.set_bank_value(ops, address, value);
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
            Self::No(r) => write!(f, "{r:?}"),
            Self::Volatile(r) => write!(f, "{r:?}"),
            Self::NonVolatile(r) => write!(f, "{r:?}"),
        }
    }
}

impl MemoryMappedBank for CartridgeRam {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_bank_value(&self, ops: &BankOps<impl PersistentStorage>, address: u16) -> u8 {
        match self {
            Self::No(r) => r.read_value(address),
            Self::Volatile(r) => r.read_value(address),
            Self::NonVolatile(r) => r.read_bank_value(ops, address),
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_bank_value(
        &mut self,
        ops: &mut BankOps<impl PersistentStorage>,
        address: u16,
        value: u8,
    ) {
        match self {
            Self::No(r) => r.set_value(address, value),
            Self::Volatile(r) => r.set_value(address, value),
            Self::NonVolatile(r) => r.set_bank_value(ops, address, value),
        }
    }
}

impl CartridgeRam {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn switch_bank(&mut self, bank: usize) {
        if let Self::Volatile(r) = self {
            r.switch_bank(bank);
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct NoRam;

impl From<NoRam> for CartridgeRam {
    fn from(c: NoRam) -> Self {
        Self::No(c)
    }
}

impl fmt::Debug for NoRam {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl MemoryMappedHardware for NoRam {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_value(&self, _address: u16) -> u8 {
        0xFF
    }
    fn set_value(&mut self, _address: u16, _value: u8) {}
}

#[derive(Clone, Serialize, Deserialize)]
struct VolatileRam {
    switchable_bank: SwitchableBank<MemoryChunk>,
}

impl From<VolatileRam> for CartridgeRam {
    fn from(c: VolatileRam) -> Self {
        Self::Volatile(c)
    }
}

impl fmt::Debug for VolatileRam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+RAM")
    }
}

impl VolatileRam {
    fn new(ram: Vec<MemoryChunk>) -> Self {
        Self {
            switchable_bank: SwitchableBank::new(ram, 0),
        }
    }

    fn switch_bank(&mut self, bank: usize) {
        self.switchable_bank.switch_bank(bank)
    }
}

impl MemoryMappedHardware for VolatileRam {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_value(&self, address: u16) -> u8 {
        self.switchable_bank.read_value(address)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_value(&mut self, address: u16, value: u8) {
        self.switchable_bank.set_value(address, value);
    }
}

#[derive(Default, Serialize, Deserialize)]
struct SramChunk {
    offset: u64,
    memory: MemoryChunk,
}

impl SramChunk {
    fn new(offset: u64, memory: MemoryChunk) -> Self {
        Self { offset, memory }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn len(&self) -> u16 {
        self.memory.len()
    }
}

impl MemoryMappedBank for SramChunk {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_bank_value(&self, _ops: &BankOps<impl PersistentStorage>, address: u16) -> u8 {
        self.memory.read_value(address)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_bank_value(
        &mut self,
        ops: &mut BankOps<impl PersistentStorage>,
        address: u16,
        value: u8,
    ) {
        self.memory.set_value(address, value);
        if let Some(file) = &mut ops.sram_file {
            file.seek(SeekFrom::Start(self.offset + address as u64))
                .unwrap();
            file.write_all(&[value]).unwrap();
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
struct NonVolatileRam(SwitchableBank<SramChunk>);

impl From<NonVolatileRam> for CartridgeRam {
    fn from(c: NonVolatileRam) -> Self {
        Self::NonVolatile(c)
    }
}

impl fmt::Debug for NonVolatileRam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+RAM+BATTERY")
    }
}

impl NonVolatileRam {
    fn new(ram: Vec<SramChunk>) -> Self {
        Self(SwitchableBank::new(ram, 0))
    }
}

impl MemoryMappedBank for NonVolatileRam {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_bank_value(&self, ops: &BankOps<impl PersistentStorage>, address: u16) -> u8 {
        self.0.read_bank_value(ops, address)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_bank_value(
        &mut self,
        ops: &mut BankOps<impl PersistentStorage>,
        address: u16,
        value: u8,
    ) {
        if address as usize >= self.0.total_len() {
            return;
        }

        self.0.set_bank_value(ops, address, value);
    }
}

pub struct GamePak<Storage: PersistentStorage> {
    title: String,
    hash: u32,
    ops: BankOps<Storage>,
    mbc: MemoryBankController,
}

impl<Storage: PersistentStorage> MemoryMappedHardware for GamePak<Storage> {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_value(&self, address: u16) -> u8 {
        self.mbc.read_bank_value(&self.ops, address)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_value(&mut self, address: u16, value: u8) {
        self.mbc.set_bank_value(&mut self.ops, address, value);
    }
}

impl<Storage: PersistentStorage> MemoryMappedHardware for &GamePak<Storage> {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_value(&self, address: u16) -> u8 {
        (*self).read_value(address)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_value(&mut self, _address: u16, _value: u8) {
        panic!("can't set_value on &GamePak")
    }
}

impl<Storage: PersistentStorage> MemoryMappedHardware for &mut GamePak<Storage> {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_value(&self, address: u16) -> u8 {
        (**self).read_value(address)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_value(&mut self, address: u16, value: u8) {
        (**self).set_value(address, value)
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

fn banks_and_chunks_from_rom(rom: &[u8]) -> (Vec<RomBank>, Vec<MemoryChunk>) {
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

    let mut banks = Vec::new();
    let mut chunks = Vec::new();
    for b in 0..number_of_banks {
        let start = b * (BANK_SIZE as usize);
        let end = start + (BANK_SIZE as usize);
        banks.push(RomBank::new(b));
        chunks.push(MemoryChunk::new(rom[start..end].to_vec()));
    }

    (banks, chunks)
}

#[derive(Copy, Clone)]
struct RamDescription {
    bank_size: u16,
    number_of_banks: usize,
}

impl RamDescription {
    fn from_bytes(ram_size: u8, controller: u8) -> Self {
        if controller == 0x6 {
            return Self {
                bank_size: 512,
                number_of_banks: 1,
            };
        }

        match ram_size {
            0 => Self {
                bank_size: 0,
                number_of_banks: 0,
            },
            // 2kB 1 Bank
            1 => Self {
                bank_size: 0x800,
                number_of_banks: 1,
            },
            // 8kB 1 Bank
            2 => Self {
                bank_size: 0x2000,
                number_of_banks: 1,
            },
            // 8kB 4 Banks = 32kB
            3 => Self {
                bank_size: 0x2000,
                number_of_banks: 4,
            },
            v => panic!("Unknown RAM byte {:x}", v),
        }
    }

    fn total_len(&self) -> u16 {
        self.bank_size * self.number_of_banks as u16
    }

    fn into_ram(self) -> Vec<MemoryChunk> {
        vec![MemoryChunk::from_range(0..self.bank_size); self.number_of_banks]
    }

    fn into_sram(self) -> Vec<SramChunk> {
        self.into_ram()
            .into_iter()
            .enumerate()
            .map(|(i, m)| SramChunk::new(i as u64 * self.bank_size as u64, m))
            .collect()
    }
}

fn load_sram_from_file(sram: &mut [SramChunk], file: &mut impl io::Read) -> io::Result<()> {
    for chunk in sram {
        file.read_exact(chunk.memory.as_mut_slice())?;
    }
    Ok(())
}

impl<Storage: PersistentStorage> GamePak<Storage> {
    pub fn from_storage(storage: &mut Storage, key: &str) -> io::Result<Self> {
        let mut rom_file = storage.open(OpenMode::Read, key)?;
        let mut rom: Vec<u8> = vec![];
        rom_file.read_to_end(&mut rom)?;
        GamePak::new(&rom, storage, Some(&format!("{key}.sav")))
    }

    pub fn from_storage_without_sav(storage: &mut Storage, key: &str) -> io::Result<Self> {
        let mut rom_file = storage.open(OpenMode::Read, key)?;
        let mut rom: Vec<u8> = vec![];
        rom_file.read_to_end(&mut rom)?;
        GamePak::new(&rom, storage, None)
    }

    pub fn new(rom: &[u8], storage: &mut Storage, sram_key: Option<&str>) -> io::Result<Self> {
        assert_eq!(rom.len() % (BANK_SIZE as usize), 0, "ROM wrong size");
        let hash = rom_hash(rom);

        let (rom_banks, rom_chunks) = banks_and_chunks_from_rom(rom);

        let title_slice = &rom[TITLE];
        let title_end = title_slice
            .iter()
            .position(|&c| c == b'\0')
            .unwrap_or(title_slice.len());
        let title = str::from_utf8(&title_slice[..title_end])
            .unwrap_or_else(|_| panic!("Malformed title {:?}", title_slice))
            .into();

        let mut sram_file = None;
        let mut get_sram = |ram_descr: RamDescription| -> io::Result<Vec<SramChunk>> {
            let mut sram = ram_descr.into_sram();
            if let Some(sram_key) = sram_key {
                let mut file = storage.open(OpenMode::ReadWrite, sram_key)?;
                file.set_len(ram_descr.total_len() as u64)?;
                load_sram_from_file(&mut sram, &mut file)?;
                sram_file = Some(file);
            }
            Ok(sram)
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

        let ram_descr = RamDescription::from_bytes(rom[RAM_SIZE_ADDRESS], rom[MBC_TYPE_ADDRESS]);
        let vram = || VolatileRam::new(ram_descr.into_ram());
        let viram = VolatileInternalRam::new;
        let nvram = NonVolatileRam::new;
        let nviram = NonVolatileInternalRam::new;

        let mbc: MemoryBankController = match rom[MBC_TYPE_ADDRESS] {
            0x00 => MemoryBankController0::new(rom_banks).into(),
            0x01 => MemoryBankController1::new(rom_banks, NoRam).into(),
            0x02 => MemoryBankController1::new(rom_banks, vram()).into(),
            0x03 => MemoryBankController1::new(rom_banks, nvram(get_sram(ram_descr)?)).into(),
            0x05 => MemoryBankController2::new(rom_banks, viram()).into(),
            0x06 => MemoryBankController2::new(rom_banks, nviram(get_sram(ram_descr)?)).into(),
            0x11 => MemoryBankController3::new(rom_banks, NoRam).into(),
            0x12 => MemoryBankController3::new(rom_banks, vram()).into(),
            0x13 => MemoryBankController3::new(rom_banks, nvram(get_sram(ram_descr)?)).into(),
            0x1b => MemoryBankController5::new(rom_banks, nvram(get_sram(ram_descr)?)).into(),
            v => panic!("Unknown Memory Bank Controller {:#x}", v),
        };

        let ops = BankOps {
            sram_file,
            rom_memory: rom_chunks,
        };

        Ok(GamePak {
            ops,
            title,
            hash,
            mbc,
        })
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn hash(&self) -> u32 {
        self.hash
    }

    pub fn save_state<W: io::Write>(&self, mut writer: W) -> super::Result<()> {
        crate::codec::serialize_into(&mut writer, &self.hash)?;
        crate::codec::serialize_into(&mut writer, &self.mbc)?;
        Ok(())
    }

    pub fn load_state<R: io::Read>(&mut self, mut reader: R) -> super::Result<()> {
        let hash: u32 = crate::codec::deserialize_from(&mut reader)?;
        assert_eq!(self.hash, hash);

        self.mbc = crate::codec::deserialize_from(&mut reader)?;
        Ok(())
    }
}

impl<Storage: PersistentStorage> fmt::Debug for GamePak<Storage> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GamePak({:?}, {:?})", self.title, self.mbc)
    }
}
