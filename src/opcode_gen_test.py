#!/usr/bin/env python
# Copyright 2017 Remi Bernotavicius

import StringIO
import unittest

from opcode_gen import OpcodeCodeGenerator

class OpcodeGenFullTest(unittest.TestCase):
    def setUp(self):
        self.outfile = StringIO.StringIO()

    def generate(self, opcode_dict):
        OpcodeCodeGenerator(self.outfile, opcode_dict, '8080', 'foo').generate()

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
        self.generate(opcode_dict)
        self.assertNotEqual(self.outfile.getvalue(), "")

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
        self.generate(opcode_dict)
        self.assertNotEqual(self.outfile.getvalue(), "")

    def test_many_args(self):
        opcode_dict = {
            "0x00": {"instr": "HELLO",
                     "description": "hello",
                     "args": ["1", "A", "D8", "D16"],
                     "size": 4},
        }
        self.generate(opcode_dict)
        self.assertNotEqual(self.outfile.getvalue(), "")

if __name__ == "__main__":
    unittest.main()
