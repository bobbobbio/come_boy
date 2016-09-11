#!/usr/bin/env python

import StringIO
import textwrap
import unittest

import opcode_gen

class OpcodeGenTest(unittest.TestCase):
    def test_read_args_with_adr(self):
        self.assertEqual(opcode_gen.read_args(['adr'], '&mut stream'),
            ['read_u16(&mut stream).unwrap()'])

    def test_read_args_with_D(self):
        self.assertEqual(opcode_gen.read_args(['D12'], '&mut stream'),
            ['read_u12(&mut stream).unwrap()'])

    def test_read_args_with_constant(self):
        self.assertEqual(opcode_gen.read_args(['12'], None), ['12 as u8'])

    def test_read_args_with_register(self):
        self.assertEqual(opcode_gen.read_args(['A'], None), ['Register8080::A'])

    def test_read_args_with_multiple(self):
        self.assertEqual(opcode_gen.read_args(['A', '12'], None),
            ['Register8080::A', '12 as u8'])

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
        opcode_gen.generate(opcode_dict, self.outfile)
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

if __name__ == "__main__":
    unittest.main()
