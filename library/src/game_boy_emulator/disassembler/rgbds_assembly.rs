// Copyright 2018 Remi Bernotavicius

use crate::emulator_common::disassembler::{InstructionPrinter, InstructionPrinterFactory};
use crate::emulator_common::Intel8080Register;
use crate::io::{self, Result};
pub use crate::lr35902_emulator::{LR35902Instruction, LR35902InstructionSet};
use core::mem;

pub struct RGBDSInstructionPrinter<'a> {
    stream_out: &'a mut dyn io::Write,
    error: Result<()>,
    address: u16,
}

#[derive(Copy, Clone)]
pub struct RGBDSInstructionPrinterFactory;

impl<'a> InstructionPrinterFactory<'a> for RGBDSInstructionPrinterFactory {
    type Output = RGBDSInstructionPrinter<'a>;
    fn create(&self, stream_out: &'a mut dyn io::Write) -> RGBDSInstructionPrinter<'a> {
        RGBDSInstructionPrinter {
            stream_out,
            error: Ok(()),
            address: 0,
        }
    }
}

impl<'a> InstructionPrinter<'a> for RGBDSInstructionPrinter<'a> {
    type Instruction = LR35902Instruction;

    fn print_instruction(&mut self, instr: LR35902Instruction, address: u16) -> Result<()> {
        self.address = address + (instr.size() as u16);
        instr.dispatch(self);
        mem::replace(&mut self.error, Ok(()))
    }

    fn get_instruction<R: io::Read>(&self, stream: R) -> Result<Option<LR35902Instruction>> {
        LR35902Instruction::from_reader(stream)
    }
}

fn str_from_register(reg: Intel8080Register) -> &'static str {
    match reg {
        Intel8080Register::B => "b",
        Intel8080Register::C => "c",
        Intel8080Register::D => "d",
        Intel8080Register::E => "e",
        Intel8080Register::H => "h",
        Intel8080Register::L => "l",
        Intel8080Register::A => "a",
        Intel8080Register::M => "[hl]",
        _ => panic!("Unknown register {:?}", reg),
    }
}

fn str_from_register_pair(reg: Intel8080Register) -> &'static str {
    match reg {
        Intel8080Register::B => "bc",
        Intel8080Register::D => "de",
        Intel8080Register::H => "hl",
        Intel8080Register::SP => "sp",
        Intel8080Register::PSW => "af",
        _ => panic!("Unknown register pair {:?}", reg),
    }
}

fn comment_from_address(address: u16) -> &'static str {
    match address {
        0xFF00 => " ; P1 Joypad",
        0xFF01 => " ; Serial Transfer Data",
        0xFF02 => " ; Serial Transfer Control",
        0xFF04 => " ; Divider",
        0xFF05 => " ; Timer Counter",
        0xFF06 => " ; Timer Modulo",
        0xFF07 => " ; Timer Control",
        0xFF0F => " ; IF",
        0xFF40 => " ; LCDC",
        0xFF41 => " ; STAT",
        0xFF42 => " ; SCY",
        0xFF43 => " ; SCX",
        0xFF44 => " ; LY",
        0xFF45 => " ; LYC",
        0xFF46 => " ; DMA",
        0xFF47 => " ; BGP",
        0xFF48 => " ; OBP0",
        0xFF49 => " ; OBP1",
        0xFF4A => " ; WY",
        0xFF4B => " ; WX",
        0xFFFF => " ; IE",
        0xFF10 => " ; Channel 1 Sweep",
        0xFF11 => " ; Channel 1 Sound Length",
        0xFF12 => " ; Channel 1 Volume Envelope",
        0xFF13 => " ; Channel 1 Frequency (low)",
        0xFF14 => " ; Channel 1 Frequency (high)",
        0xFF16 => " ; Channel 2 Sound Length",
        0xFF17 => " ; Channel 2 Volume Envelope",
        0xFF18 => " ; Channel 2 Frequency (low)",
        0xFF19 => " ; Channel 2 Frequency (high)",
        0xFF1A => " ; Channel 3 Enabled",
        0xFF1B => " ; Channel 3 Sound Length",
        0xFF1C => " ; Channel 3 Output Level",
        0xFF1D => " ; Channel 3 Frequency (low)",
        0xFF1E => " ; Channel 3 Frequency (high)",
        0xFF20 => " ; Channel 4 Sound Length",
        0xFF21 => " ; Channel 4 Volume Envelope",
        0xFF22 => " ; Channel 4 Polynomial Counter",
        0xFF23 => " ; Channel 4 Counter",
        0xFF24 => " ; Channel Control",
        0xFF25 => " ; Sound Output Terminal",
        0xFF26 => " ; Sound Enabled",
        _ => "",
    }
}

impl<'a> LR35902InstructionSet for RGBDSInstructionPrinter<'a> {
    fn reset_bit(&mut self, implicit_data1: u8, register2: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {},{}",
            "res",
            implicit_data1,
            str_from_register(register2)
        );
    }
    fn load_sp_from_h_and_l(&mut self) {
        self.error = write!(self.stream_out, "{:04} sp,hl", "ld");
    }
    fn shift_register_right(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "srl",
            str_from_register(register1)
        );
    }
    fn double_add(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} hl,{}",
            "add",
            str_from_register_pair(register1)
        );
    }
    fn or_immediate_with_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} a,${data1:02X}", "or");
    }
    fn no_operation(&mut self) {
        self.error = write!(self.stream_out, "nop");
    }
    fn rotate_register_left_through_carry(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "rl",
            str_from_register(register1)
        );
    }
    fn load_register_pair_immediate(&mut self, register1: Intel8080Register, data2: u16) {
        self.error = write!(
            self.stream_out,
            "{:04} {},${:04X}{}",
            "ld",
            str_from_register_pair(register1),
            data2,
            comment_from_address(data2),
        );
    }
    fn move_data(&mut self, register1: Intel8080Register, register2: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {},{}",
            "ld",
            str_from_register(register1),
            str_from_register(register2)
        );
    }
    fn enable_interrupts(&mut self) {
        self.error = write!(self.stream_out, "ei");
    }
    fn return_if_zero(&mut self) {
        self.error = write!(self.stream_out, "{:04} z", "ret");
    }
    fn exclusive_or_immediate_with_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} a,${data1:02X}", "xor");
    }
    fn rotate_accumulator_right(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "rrca");
    }
    fn and_immediate_with_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} a,${data1:02X}", "and");
    }
    fn decrement_register_or_memory(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "dec",
            str_from_register(register1)
        );
    }
    fn halt(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "halt");
    }
    fn return_and_enable_interrupts(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "reti");
    }
    fn set_bit(&mut self, implicit_data1: u8, register2: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {},{}",
            "set",
            implicit_data1,
            str_from_register(register2)
        );
    }
    fn rotate_register_right(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "rr",
            str_from_register(register1)
        );
    }
    fn shift_register_right_signed(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "sra",
            str_from_register(register1)
        );
    }
    fn compare_with_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "cp",
            str_from_register(register1)
        );
    }
    fn restart(&mut self, implicit_data1: u8) {
        self.error = write!(
            self.stream_out,
            "{:04} ${:02X}",
            "rst",
            (implicit_data1 as u16) << 3
        );
    }
    fn jump_relative_if_not_zero(&mut self, data1: u8) {
        let a = self.address.wrapping_add((data1 as i8) as u16);
        self.error = write!(self.stream_out, "{:04} nz,${a:04X}", "jr");
    }
    fn rotate_register_left(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "rl",
            str_from_register(register1)
        );
    }
    fn decrement_register_pair(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "dec",
            str_from_register_pair(register1)
        );
    }
    fn complement_carry(&mut self) {
        self.error = write!(self.stream_out, "ccf");
    }
    fn load_accumulator_direct(&mut self, address1: u16) {
        self.error = write!(
            self.stream_out,
            "{:04} a,[${:04X}]{}",
            "ld",
            address1,
            comment_from_address(address1)
        );
    }
    fn return_if_not_zero(&mut self) {
        self.error = write!(self.stream_out, "{:04} nz", "ret");
    }
    fn logical_or_with_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "or",
            str_from_register(register1)
        );
    }
    fn shift_register_left(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "sla",
            str_from_register(register1)
        );
    }
    fn jump(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${address1:04X}", "jp");
    }
    fn call_if_not_zero(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} nz,${address1:04X}", "call");
    }
    fn store_sp_direct(&mut self, address1: u16) {
        self.error = write!(
            self.stream_out,
            "{:04} [${:04X}],sp{}",
            "ld",
            address1,
            comment_from_address(address1)
        );
    }
    fn subtract_immediate_from_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} a,${data1:02X}", "sub");
    }
    fn rotate_accumulator_left_through_carry(&mut self) {
        self.error = write!(self.stream_out, "rla");
    }
    fn subtract_from_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "sub",
            str_from_register(register1)
        );
    }
    fn load_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} a,[{}]",
            "ld",
            str_from_register_pair(register1),
        );
    }
    fn move_and_decrement_hl(
        &mut self,
        register1: Intel8080Register,
        register2: Intel8080Register,
    ) {
        self.error = write!(
            self.stream_out,
            "{:04} {},{}",
            "ldd",
            str_from_register(register1),
            str_from_register(register2)
        );
    }
    fn jump_relative_if_no_carry(&mut self, data1: u8) {
        let a = self.address.wrapping_add((data1 as i8) as u16);
        self.error = write!(self.stream_out, "{:04} nc,${a:04X}", "jr");
    }
    fn return_unconditionally(&mut self) {
        self.error = write!(self.stream_out, "ret");
    }
    fn load_accumulator_one_byte(&mut self) {
        self.error = write!(self.stream_out, "{:04} a,[$FF00+c]", "ld");
    }
    fn jump_if_not_zero(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} nz,${address1:04X}", "jp");
    }
    fn jump_relative_if_carry(&mut self, data1: u8) {
        let a = self.address.wrapping_add((data1 as i8) as u16);
        self.error = write!(self.stream_out, "{:04} c,${a:04X}", "jr");
    }
    fn call_if_carry(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} c,${address1:04X}", "call");
    }
    fn test_bit(&mut self, implicit_data1: u8, register2: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {},{}",
            "bit",
            implicit_data1,
            str_from_register(register2)
        );
    }
    fn rotate_accumulator_right_through_carry(&mut self) {
        self.error = write!(self.stream_out, "rra");
    }
    fn store_accumulator_direct_one_byte(&mut self, data1: u8) {
        self.error = write!(
            self.stream_out,
            "{:04} [$FF{:02X}],a{}",
            "ldh",
            data1,
            comment_from_address(0xFF00 + data1 as u16)
        );
    }
    fn logical_and_with_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "and",
            str_from_register(register1)
        );
    }
    fn halt_until_button_press(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "stop");
    }
    fn jump_relative(&mut self, data1: u8) {
        let a = self.address.wrapping_add((data1 as i8) as u16);
        self.error = write!(self.stream_out, "{:04} ${a:04X}", "jr");
    }
    fn store_accumulator_one_byte(&mut self) {
        self.error = write!(self.stream_out, "{:04} [$FF00+c],a", "ld");
    }
    fn set_carry(&mut self) {
        self.error = write!(self.stream_out, "scf");
    }
    fn jump_if_no_carry(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} nc,${address1:04X}", "jp");
    }
    fn call(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} ${address1:04X}", "call");
    }
    fn return_if_no_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04} nc", "ret");
    }
    fn call_if_zero(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} z,${address1:04X}", "call");
    }
    fn load_accumulator_direct_one_byte(&mut self, data1: u8) {
        self.error = write!(
            self.stream_out,
            "{:04} a,[$FF{:02X}]{}",
            "ldh",
            data1,
            comment_from_address(0xFF00 + data1 as u16)
        );
    }
    fn jump_if_carry(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} c,${address1:04X}", "jp");
    }
    fn rotate_register_right_through_carry(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "rr",
            str_from_register(register1)
        );
    }
    fn add_immediate_to_accumulator_with_carry(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} a,${data1:02X}", "adc");
    }
    fn store_accumulator_direct(&mut self, address1: u16) {
        self.error = write!(
            self.stream_out,
            "{:04} [${:04X}],a{}",
            "ld",
            address1,
            comment_from_address(address1)
        );
    }
    fn swap_register(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "swap",
            str_from_register(register1)
        );
    }
    fn increment_register_pair(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "inc",
            str_from_register_pair(register1)
        );
    }
    fn store_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} [{}],a",
            "ld",
            str_from_register_pair(register1)
        );
    }
    fn add_to_accumulator_with_carry(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "adc",
            str_from_register(register1)
        );
    }
    fn subtract_from_accumulator_with_borrow(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "sbc",
            str_from_register(register1)
        );
    }
    fn push_data_onto_stack(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "push",
            str_from_register_pair(register1)
        );
    }
    fn increment_register_or_memory(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "inc",
            str_from_register(register1)
        );
    }
    fn load_program_counter(&mut self) {
        self.error = write!(self.stream_out, "{:04} [hl]", "jp");
    }
    fn pop_data_off_stack(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "pop",
            str_from_register_pair(register1)
        );
    }
    fn add_immediate_to_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} a,${data1:02X}", "add");
    }
    fn store_sp_plus_immediate(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} hl,[sp+${data1:02X}]", "ld");
    }
    fn complement_accumulator(&mut self) {
        self.error = write!(self.stream_out, "cpl");
    }
    fn move_and_increment_hl(
        &mut self,
        register1: Intel8080Register,
        register2: Intel8080Register,
    ) {
        self.error = write!(
            self.stream_out,
            "{:04} {},{}",
            "ldi",
            str_from_register(register1),
            str_from_register(register2)
        );
    }
    fn logical_exclusive_or_with_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "xor",
            str_from_register(register1)
        );
    }
    fn add_immediate_to_sp(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} sp,${data1:02X}", "add");
    }
    fn add_to_accumulator(&mut self, register1: Intel8080Register) {
        self.error = write!(
            self.stream_out,
            "{:04} {}",
            "add",
            str_from_register(register1)
        );
    }
    fn disable_interrupts(&mut self) {
        self.error = write!(self.stream_out, "di");
    }
    fn compare_immediate_with_accumulator(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} a,${data1:02X}", "cp");
    }
    fn decimal_adjust_accumulator(&mut self) {
        self.error = write!(self.stream_out, "daa");
    }
    fn move_immediate_data(&mut self, register1: Intel8080Register, data2: u8) {
        self.error = write!(
            self.stream_out,
            "{:04} {},${:02X}",
            "ld",
            str_from_register(register1),
            data2
        );
    }
    fn call_if_no_carry(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} nc,${address1:04X}", "call");
    }
    fn jump_relative_if_zero(&mut self, data1: u8) {
        let a = self.address.wrapping_add((data1 as i8) as u16);
        self.error = write!(self.stream_out, "{:04} z,${a:04X}", "jr");
    }
    fn return_if_carry(&mut self) {
        self.error = write!(self.stream_out, "{:04} c", "ret");
    }
    fn jump_if_zero(&mut self, address1: u16) {
        self.error = write!(self.stream_out, "{:04} z,${address1:04X}", "jp");
    }
    fn subtract_immediate_from_accumulator_with_borrow(&mut self, data1: u8) {
        self.error = write!(self.stream_out, "{:04} a,${data1:02X}", "sbc");
    }
    fn rotate_accumulator_left(&mut self) {
        self.error = write!(self.stream_out, "{:04}", "rlca");
    }
}
