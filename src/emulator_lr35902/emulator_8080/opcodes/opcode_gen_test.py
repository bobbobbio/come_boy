#!/usr/bin/env python

import unittest
import opcode_gen

class OpcodeGenTest(unittest.TestCase):
    def test_read_args_with_adr(self):
        self.assertEqual(opcode_gen.read_args(['adr'], '&mut stream'),
            ['read_u16(&mut stream).ok().expect("")'])

    def test_read_args_with_D(self):
        self.assertEqual(opcode_gen.read_args(['D12'], '&mut stream'),
            ['read_u12(&mut stream).ok().expect("")'])

    def test_read_args_with_constant(self):
        self.assertEqual(opcode_gen.read_args(['12'], None), ['12 as u8'])

    def test_read_args_with_register(self):
        self.assertEqual(opcode_gen.read_args(['A'], None), ['Register8080::A'])

    def test_read_args_with_multiple(self):
        self.assertEqual(opcode_gen.read_args(['A', '12'], None),
            ['Register8080::A', '12 as u8'])

if __name__ == "__main__":
    unittest.main()
