// Copyright 2017 Remi Bernotavicius

use serde_derive::{Deserialize, Serialize};

pub mod debugger;
pub mod disassembler;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryDescription {
    Instruction,
    Data(u16),
    Ascii(u16),
}

pub trait MemoryAccessor {
    fn read_memory(&self, address: u16) -> u8;
    fn set_memory(&mut self, address: u16, value: u8);

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_memory_u16(&self, address: u16) -> u16 {
        if address == 0xFFFF {
            return self.read_memory(address) as u16;
        }

        (self.read_memory(address + 1) as u16) << 8 | (self.read_memory(address) as u16)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_memory_u16(&mut self, address: u16, value: u16) {
        self.set_memory(address, value as u8);

        if address == 0xFFFF {
            return;
        }

        self.set_memory(address + 1, (value >> 8) as u8);
    }

    fn set_interrupts_enabled(&mut self, enabled: bool);

    fn describe_address(&self, address: u16) -> MemoryDescription;
}

impl MemoryAccessor for &dyn MemoryAccessor {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_memory(&self, address: u16) -> u8 {
        (*self).read_memory(address)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_memory(&mut self, _address: u16, _value: u8) {
        unreachable!()
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_interrupts_enabled(&mut self, _enabled: bool) {
        unreachable!()
    }

    fn describe_address(&self, address: u16) -> MemoryDescription {
        (*self).describe_address(address)
    }
}

pub struct SimpleMemoryAccessor {
    pub memory: [u8; 0x10000],
    pub interrupts_enabled: bool,
}

impl Default for SimpleMemoryAccessor {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleMemoryAccessor {
    pub fn new() -> Self {
        Self {
            memory: [0; 0x10000],
            interrupts_enabled: false,
        }
    }
}

impl MemoryAccessor for SimpleMemoryAccessor {
    fn read_memory(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn set_memory(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    fn set_interrupts_enabled(&mut self, enabled: bool) {
        self.interrupts_enabled = enabled;
    }

    fn describe_address(&self, _address: u16) -> MemoryDescription {
        MemoryDescription::Instruction
    }
}

impl MemoryAccessor for [u8] {
    fn read_memory(&self, address: u16) -> u8 {
        self[address as usize]
    }

    fn set_memory(&mut self, address: u16, value: u8) {
        self[address as usize] = value;
    }

    fn set_interrupts_enabled(&mut self, _enabled: bool) {}

    fn describe_address(&self, _address: u16) -> MemoryDescription {
        MemoryDescription::Instruction
    }
}

/*  ___       _       _  ___   ___   ___   ___  ____            _     _
 * |_ _|_ __ | |_ ___| |( _ ) / _ \ ( _ ) / _ \|  _ \ ___  __ _(_)___| |_ ___ _ __
 *  | || '_ \| __/ _ \ |/ _ \| | | |/ _ \| | | | |_) / _ \/ _` | / __| __/ _ \ '__|
 *  | || | | | ||  __/ | (_) | |_| | (_) | |_| |  _ <  __/ (_| | \__ \ ||  __/ |
 * |___|_| |_|\__\___|_|\___/ \___/ \___/ \___/|_| \_\___|\__, |_|___/\__\___|_|
 *                                                        |___/
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Intel8080Register {
    B = 0,
    C = 1,
    D = 2,
    E = 3,
    H = 4,
    L = 5,
    A = 6,

    // Conatins all of the condition bits.
    FLAGS = 7,

    // Stack Pointer (2 bytes)
    SP = 8,

    // Special fake register called 'Program Status Word'. It refers to register pair, A and
    // FLAGS.
    PSW = 10,

    // Special fake register called 'Memory'. Represents the data stored at address contained in
    // HL.
    M = 11,

    Count = 12,
}
