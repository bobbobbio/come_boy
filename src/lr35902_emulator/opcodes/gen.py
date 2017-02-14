#!/usr/bin/env python
# Copyright 2017 Remi Bernotavicius

import os
import opcode_gen

def main():
    opcode_gen.generate_opcode_rs(
        path=os.path.dirname(os.path.realpath(__file__)),
        instruction_set_name='LR35902')

if __name__ == '__main__':
    main()
