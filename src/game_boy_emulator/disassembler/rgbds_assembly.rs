use std::{mem, io};
use std::io::Result;
use emulator_common::{InstructionPrinter, InstructionPrinterFactory, Intel8080Register};
pub use lr35902_emulator::{
    dispatch_lr35902_instruction, get_lr35902_instruction, LR35902InstructionSet};

pub struct RGBDSInstructionPrinter<'a> {
    stream_out: &'a mut io::Write,
    error: Result<()>
}

pub struct RGBDSInstructionPrinterFactory;

impl<'a> InstructionPrinterFactory<'a> for RGBDSInstructionPrinterFactory {
    type Output = RGBDSInstructionPrinter<'a>;
    fn new(&self, stream_out: &'a mut io::Write) -> RGBDSInstructionPrinter<'a>
    {
        return RGBDSInstructionPrinter {
            stream_out: stream_out,
            error: Ok(())
        };
    }
}

impl<'a> InstructionPrinter<'a> for RGBDSInstructionPrinter<'a> {
    fn print_instruction(&mut self, stream: &[u8]) -> Result<()>
    {
        dispatch_lr35902_instruction(stream, self);
        mem::replace(&mut self.error, Ok(()))
    }
    fn get_instruction(&self, stream: &[u8]) -> Option<Vec<u8>>
    {
        get_lr35902_instruction(stream)
    }
}

fn str_from_register(reg: Intel8080Register) -> &'static str
{
    match reg {
        Intel8080Register::B => "b",
        Intel8080Register::C => "c",
        Intel8080Register::D => "d",
        Intel8080Register::E => "e",
        Intel8080Register::H => "h",
        Intel8080Register::L => "l",
        Intel8080Register::A => "a",
        Intel8080Register::M => "[hl]",
        _ => panic!("Unknown register {:?}", reg)
    }
}

fn str_from_register_pair(reg: Intel8080Register) -> &'static str
{
    match reg {
        Intel8080Register::B => "bc",
        Intel8080Register::D => "de",
        Intel8080Register::H => "hl",
        Intel8080Register::SP => "sp",
        Intel8080Register::PSW => "af",
        _ => panic!("Unknown register pair {:?}", reg)
    }
}

impl<'a> LR35902InstructionSet for RGBDSInstructionPrinter<'a> {
    fn reset_bit(&mut self, implicit_data1: u8, register2: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {},{}", "res", implicit_data1, str_from_register(register2));
    }
    fn load_sp_from_h_and_l(&mut self)
    {
        self.error = write!(self.stream_out, "{:04} sp,hl", "ld");
    }
    fn shift_register_right(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "srl", str_from_register(register1));
    }
    fn double_add(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} hl,{}", "add", str_from_register_pair(register1));
    }
    fn or_immediate_with_accumulator(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} a,${:02x}", "or", data1);
    }
    fn no_operation(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "nop");
    }
    fn rotate_register_left_through_carry(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "rlc", str_from_register(register1));
    }
    fn load_register_pair_immediate(&mut self, register1: Intel8080Register, data2: u16)
    {
        self.error = write!(self.stream_out, "{:04} {},${:04x}", "ld", str_from_register_pair(register1), data2);
    }
    fn move_data(&mut self, register1: Intel8080Register, register2: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {},{}", "ld", str_from_register(register1), str_from_register(register2));
    }
    fn enable_interrupts(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "ei");
    }
    fn return_if_zero(&mut self)
    {
        self.error = write!(self.stream_out, "{:04} z", "ret");
    }
    fn exclusive_or_immediate_with_accumulator(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} a,${:02x}", "xor", data1);
    }
    fn rotate_accumulator_right(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "rrca");
    }
    fn and_immediate_with_accumulator(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} a,${:02x}", "and", data1);
    }
    fn decrement_register_or_memory(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "dec", str_from_register(register1));
    }
    fn halt(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "halt");
    }
    fn return_and_enable_interrupts(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "reti");
    }
    fn set_bit(&mut self, implicit_data1: u8, register2: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {},{}", "set", implicit_data1, str_from_register(register2));
    }
    fn rotate_register_right(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "rr", str_from_register(register1));
    }
    fn shift_register_right_signed(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "sra", str_from_register(register1));
    }
    fn compare_with_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "cp", str_from_register(register1));
    }
    fn restart(&mut self, implicit_data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "rst", implicit_data1);
    }
    fn jump_relative_if_not_zero(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} nz,${:02x}", "jr", data1);
    }
    fn rotate_register_left(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "rl", str_from_register(register1));
    }
    fn decrement_register_pair(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "dec", str_from_register_pair(register1));
    }
    fn complement_carry(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "ccf");
    }
    fn load_accumulator_direct(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} a,[${:04x}]", "ld", address1);
    }
    fn return_if_not_zero(&mut self)
    {
        self.error = write!(self.stream_out, "{:04} nz", "ret");
    }
    fn logical_or_with_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "ora", str_from_register(register1));
    }
    fn shift_register_left(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "sla", str_from_register(register1));
    }
    fn jump(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:04x}", "jp", address1);
    }
    fn call_if_not_zero(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:04x}", "cnz", address1);
    }
    fn store_sp_direct(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:04x}", "sspd", address1);
    }
    fn subtract_immediate_from_accumulator(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "sui", data1);
    }
    fn rotate_accumulator_left_through_carry(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "rla");
    }
    fn subtract_from_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "sub", str_from_register(register1));
    }
    fn load_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "ldax", str_from_register(register1));
    }
    fn move_and_decrement_hl(&mut self, register1: Intel8080Register, register2: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {} {}", "ldd", str_from_register(register1), str_from_register(register2));
    }
    fn jump_relative_if_no_carry(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "jrnc", data1);
    }
    fn return_unconditionally(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "ret");
    }
    fn load_accumulator_one_byte(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "ldac");
    }
    fn jump_if_not_zero(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:04x}", "jnz", address1);
    }
    fn jump_relative_if_carry(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "jrc", data1);
    }
    fn call_if_carry(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:04x}", "cc", address1);
    }
    fn test_bit(&mut self, implicit_data1: u8, register2: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x} {}", "bit", implicit_data1, str_from_register(register2));
    }
    fn rotate_accumulator_right_through_carry(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "rra");
    }
    fn store_accumulator_direct_one_byte(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} [$ff{:02x}],a", "ldh", data1);
    }
    fn logical_and_with_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "and", str_from_register(register1));
    }
    fn halt_until_button_press(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "stop");
    }
    fn jump_relative(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "jr", data1);
    }
    fn store_accumulator_one_byte(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "[$ff00+c],a");
    }
    fn set_carry(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "scf");
    }
    fn jump_if_no_carry(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:04x}", "jnc", address1);
    }
    fn call(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:04x}", "call", address1);
    }
    fn return_if_no_carry(&mut self)
    {
        self.error = write!(self.stream_out, "{:04} {}", "ret", "nc");
    }
    fn call_if_zero(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:04x}", "cz", address1);
    }
    fn load_accumulator_direct_one_byte(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} a,[$ff{:02x}]", "ldh", data1);
    }
    fn jump_if_carry(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} ${:04x}", "jc", address1);
    }
    fn rotate_register_right_through_carry(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "rr", str_from_register(register1));
    }
    fn add_immediate_to_accumulator_with_carry(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} ${:02x}", "aci", data1);
    }
    fn store_accumulator_direct(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} [${:04x}],a", "ld", address1);
    }
    fn swap_register(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "swap", str_from_register(register1));
    }
    fn increment_register_pair(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "inx", str_from_register_pair(register1));
    }
    fn store_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} [{}],a", "ldd", str_from_register_pair(register1));
    }
    fn add_to_accumulator_with_carry(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "adc", str_from_register(register1));
    }
    fn subtract_from_accumulator_with_borrow(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "sbc", str_from_register(register1));
    }
    fn push_data_onto_stack(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "push", str_from_register_pair(register1));
    }
    fn increment_register_or_memory(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "inc", str_from_register(register1));
    }
    fn load_program_counter(&mut self)
    {
        self.error = write!(self.stream_out, "{:04} [hl]", "jp");
    }
    fn pop_data_off_stack(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "pop", str_from_register_pair(register1));
    }
    fn add_immediate_to_accumulator(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} a,${:02x}", "add", data1);
    }
    fn store_sp_plus_immediate(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} hl,[sp+${:02x}]", "ld", data1);
    }
    fn complement_accumulator(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "cpl");
    }
    fn move_and_increment_hl(&mut self, register1: Intel8080Register, register2: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {} {}", "ldi", str_from_register(register1), str_from_register(register2));
    }
    fn logical_exclusive_or_with_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "xor", str_from_register(register1));
    }
    fn add_immediate_to_sp(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} sp,${:02x}", "add", data1);
    }
    fn add_to_accumulator(&mut self, register1: Intel8080Register)
    {
        self.error = write!(self.stream_out, "{:04} {}", "add", str_from_register(register1));
    }
    fn disable_interrupts(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "di");
    }
    fn compare_immediate_with_accumulator(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} a,${:02x}", "cp", data1);
    }
    fn decimal_adjust_accumulator(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "daa");
    }
    fn move_immediate_data(&mut self, register1: Intel8080Register, data2: u8)
    {
        self.error = write!(self.stream_out, "{:04} {},${:02x}", "ld", str_from_register(register1), data2);
    }
    fn call_if_no_carry(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} nc,${:04x}", "call", address1);
    }
    fn jump_relative_if_zero(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} z,${:02x}", "jr", data1);
    }
    fn return_if_carry(&mut self)
    {
        self.error = write!(self.stream_out, "{:04} c", "ret");
    }
    fn jump_if_zero(&mut self, address1: u16)
    {
        self.error = write!(self.stream_out, "{:04} z,${:04x}", "jp", address1);
    }
    fn subtract_immediate_from_accumulator_with_borrow(&mut self, data1: u8)
    {
        self.error = write!(self.stream_out, "{:04} a,${:02x}", "sub", data1);
    }
    fn rotate_accumulator_left(&mut self)
    {
        self.error = write!(self.stream_out, "{:04}", "rlca");
    }
}
