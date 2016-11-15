#!/usr/bin/env python

import StringIO
import textwrap
import unittest

from opcode_gen import OpcodeCodeGenerator

class OpcodeGenFullTest(unittest.TestCase):
    def setUp(self):
        self.outfile = StringIO.StringIO()

    def test_opcodes_no_args(self):
        opcode_dict = {
            "0x00": {"instr": "HELLO",
                     "description": "print hello",
                     "args": [],
                     "size": 1},
            "0x01": {"instr": "GOODBYE",
                     "description": "print goodbye",
                     "args": [],
                     "size": 1}
        }
        OpcodeCodeGenerator(self.outfile, opcode_dict, '8080').generate()
        self.assertEqual(self.outfile.getvalue(), textwrap.dedent('''
        use emulator_lr35902::emulator_8080::opcodes::{
            read_u16, read_u8, Register8080, OpcodePrinter8080};

        /*
         * Warning: This file is generated.  Don't manually edit.
         * Instead edit opcodes/opcode_gen.py
         */

        pub trait InstructionSet8080 {
            fn print_hello(&mut self);
            fn print_goodbye(&mut self);
        }

        pub fn dispatch_8080_opcode<I: InstructionSet8080>(
            mut stream: &[u8],
            machine: &mut I)
        {
            match read_u8(&mut stream).unwrap() {
                0x00 => machine.print_hello(),
                0x01 => machine.print_goodbye(),

                _ => panic!("Unknown opcode")
           };
        }

        pub fn get_8080_opcode_size(opcode: u8) -> u8
        {
            match opcode {
                0x00 => 1,
                0x01 => 1,

                _ => panic!("Unknown opcode")
           }
        }

        impl<'a> InstructionSet8080 for OpcodePrinter8080<'a> {
            fn print_hello(&mut self)
            {
                write!(self.stream_out, "{:04}", "HELLO").unwrap();
            }
            fn print_goodbye(&mut self)
            {
                write!(self.stream_out, "{:04}", "GOODBYE").unwrap();
            }
        }'''))

    def test_opcodes_one_arg(self):
        opcode_dict = {
            "0x00": {"instr": "HELLO",
                     "description": "hello",
                     "args": ["1"],
                     "size": 1},
            "0x01": {"instr": "HELLO",
                     "description": "hello",
                     "args": ["2"],
                     "size": 1}
        }
        OpcodeCodeGenerator(self.outfile, opcode_dict, '8080').generate()
        self.assertEqual(self.outfile.getvalue(), textwrap.dedent('''
        use emulator_lr35902::emulator_8080::opcodes::{
            read_u16, read_u8, Register8080, OpcodePrinter8080};

        /*
         * Warning: This file is generated.  Don't manually edit.
         * Instead edit opcodes/opcode_gen.py
         */

        pub trait InstructionSet8080 {
            fn hello(&mut self, implicit_data1: u8);
        }

        pub fn dispatch_8080_opcode<I: InstructionSet8080>(
            mut stream: &[u8],
            machine: &mut I)
        {
            match read_u8(&mut stream).unwrap() {
                0x00 => machine.hello(1 as u8),
                0x01 => machine.hello(2 as u8),

                _ => panic!("Unknown opcode")
           };
        }

        pub fn get_8080_opcode_size(opcode: u8) -> u8
        {
            match opcode {
                0x00 => 1,
                0x01 => 1,

                _ => panic!("Unknown opcode")
           }
        }

        impl<'a> InstructionSet8080 for OpcodePrinter8080<'a> {
            fn hello(&mut self, implicit_data1: u8)
            {
                write!(self.stream_out, "{:04}", "HELLO").unwrap();
                write!(self.stream_out, " {}", implicit_data1).unwrap();
            }
        }'''))

    def test_many_args(self):
        opcode_dict = {
            "0x00": {"instr": "HELLO",
                     "description": "hello",
                     "args": ["1", "A", "D8", "D16"],
                     "size": 4},
        }
        OpcodeCodeGenerator(self.outfile, opcode_dict, '8080').generate()
        self.assertEqual(self.outfile.getvalue(), textwrap.dedent('''
        use emulator_lr35902::emulator_8080::opcodes::{
            read_u16, read_u8, Register8080, OpcodePrinter8080};

        /*
         * Warning: This file is generated.  Don't manually edit.
         * Instead edit opcodes/opcode_gen.py
         */

        pub trait InstructionSet8080 {
            fn hello(&mut self, implicit_data1: u8, register2: Register8080, data3: u8, data4: u16);
        }

        pub fn dispatch_8080_opcode<I: InstructionSet8080>(
            mut stream: &[u8],
            machine: &mut I)
        {
            match read_u8(&mut stream).unwrap() {
                0x00 => machine.hello(1 as u8, Register8080::A, read_u8(&mut stream).unwrap(), read_u16(&mut stream).unwrap()),

                _ => panic!("Unknown opcode")
           };
        }

        pub fn get_8080_opcode_size(opcode: u8) -> u8
        {
            match opcode {
                0x00 => 4,

                _ => panic!("Unknown opcode")
           }
        }

        impl<'a> InstructionSet8080 for OpcodePrinter8080<'a> {
            fn hello(&mut self, implicit_data1: u8, register2: Register8080, data3: u8, data4: u16)
            {
                write!(self.stream_out, "{:04}", "HELLO").unwrap();
                write!(self.stream_out, " {}", implicit_data1).unwrap();
                write!(self.stream_out, " {:?}", register2).unwrap();
                write!(self.stream_out, " #${:02x}", data3).unwrap();
                write!(self.stream_out, " #${:02x}", data4).unwrap();
            }
        }'''))

if __name__ == "__main__":
    unittest.main()
