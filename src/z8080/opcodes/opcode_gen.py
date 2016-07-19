#!/usr/bin/env python

import json
import textwrap
import os

def read_args(args):
    r_args = []
    for arg in args:
        if arg == 'adr':
            r_args.append(('try!(read_u16(&mut stream))'))
        elif arg.startswith('D') and arg != 'D':
            r_args.append('try!(read_u{}(&mut stream))'.format(arg[1:]))
        elif all([c.isdigit() for c in arg]):
            r_args.append('{} as u8'.format(arg))
        else:
            assert all([c.isalpha() for c in arg])
            r_args.append('Register8080::' + arg)
    return r_args

def name_args(args):
    return ['{}{}: {}'.format(arg[0], num + 1, arg[1])
        for num, arg in enumerate(args)]

def main():
    script_dir = os.path.dirname(os.path.realpath(__file__))

    with open(os.path.join(script_dir, 'opcodes.json')) as f:
        opcode_dict = json.loads(f.read())
    output_file = os.path.join(script_dir, 'opcode_gen.rs')

    with open(output_file, 'w') as out_file:
        out_file.write(textwrap.dedent('''
            use std::io::{self, Result};

            /*
             * Warning: This file is generated.  Don't manually edit.
             * Instead edit opcodes/opcode_gen.py
             */

            fn read_u16<T: io::Read>(
                mut stream: T) -> Result<u16>
            {
                let mut narg : u16;
                let mut arg_buffer = [0; 1];
                try!(stream.read_exact(&mut arg_buffer));
                narg = arg_buffer[0] as u16;
                try!(stream.read_exact(&mut arg_buffer));
                narg |= (arg_buffer[0] as u16) << 8;
                Ok(narg)
            }

            fn read_u8<T: io::Read>(
                mut stream: T) -> Result<u8>
            {
                let mut arg_buffer = [0; 1];
                try!(stream.read_exact(&mut arg_buffer));
                Ok(arg_buffer[0])
            }
        '''))

        #   __                  _   _               _        _     _
        #  / _|_   _ _ __   ___| |_(_) ___  _ __   | |_ __ _| |__ | | ___
        # | |_| | | | '_ \ / __| __| |/ _ \| '_ \  | __/ _` | '_ \| |/ _ \
        # |  _| |_| | | | | (__| |_| | (_) | | | | | || (_| | |_) | |  __/
        # |_|  \__,_|_| |_|\___|\__|_|\___/|_| |_|  \__\__,_|_.__/|_|\___|
        #

        functions = {}
        registers = set()
        for info in opcode_dict.itervalues():
            name = info['instr']
            if name == '-':
                name = 'not_implemented'

            arg_desc = []
            for arg in info['args']:
                if arg == 'adr':
                    arg_desc.append(('address', 'u16'))
                elif arg.startswith('D') and arg != 'D':
                    arg_desc.append(('data', 'u' + arg[1:]))
                elif all([c.isdigit() for c in arg]):
                    arg_desc.append(('implicit_data', 'u8'))
                else:
                    assert all([c.isalpha() for c in arg])
                    arg_desc.append(('register', 'Register8080'))
                    registers.add(arg)

            if name not in functions:
                functions[name] = arg_desc
            else:
                assert functions[name] == arg_desc, \
                    "{} has non consistant arugments".format(name)

        #                 _     _
        #  _ __ ___  __ _(_)___| |_ ___ _ __ ___
        # | '__/ _ \/ _` | / __| __/ _ \ '__/ __|
        # | | |  __/ (_| | \__ \ ||  __/ |  \__ \
        # |_|  \___|\__, |_|___/\__\___|_|  |___/
        #           |___/

        out_file.write(textwrap.dedent('''
          #[derive(Debug)]
          pub enum Register8080 {
        '''))
        for register_name in sorted(registers):
            out_file.write('    ' + register_name + ',\n')
        out_file.write('}\n')

        #  _           _                   _   _
        # (_)_ __  ___| |_ _ __ _   _  ___| |_(_) ___  _ __  ___
        # | | '_ \/ __| __| '__| | | |/ __| __| |/ _ \| '_ \/ __|
        # | | | | \__ \ |_| |  | |_| | (__| |_| | (_) | | | \__ \
        # |_|_| |_|___/\__|_|   \__,_|\___|\__|_|\___/|_| |_|___/
        #  _             _ _
        # | |_ _ __ __ _(_) |_
        # | __| '__/ _` | | __|
        # | |_| | | (_| | | |_
        #  \__|_|  \__,_|_|\__|

        out_file.write(textwrap.dedent('''
          pub trait InstructionSet8080 {
        '''))

        for name, args in functions.iteritems():
            out_file.write('    '
                'fn instruction_{}({}) -> Result<()>;\n'.format(
                    name.lower(),
                    ', '.join(['&mut self'] + name_args(args))))

        out_file.write('}\n')

        out_file.write(textwrap.dedent('''
            pub fn dispatch_opcode<I: InstructionSet8080>(
                mut stream: &[u8],
                machine: &mut I) -> Result<(u8)>
            {
                let size;
                match try!(read_u8(&mut stream)) {
        '''))

        for opcode, info in opcode_dict.iteritems():
            out_file.write('        {} => {{\n            '.format(opcode))
            name = info['instr'].lower()
            if name == '-':
                name = 'not_implemented'
            out_file.write('try!(machine.instruction_{}({})); '.format(
                name, ', '.join(read_args(info['args']))))
            out_file.write('size = {}\n        }}\n'.format(info['size']))

        out_file.write(textwrap.dedent('''
                   _ => panic!("Unknown opcode")
              };
              Ok(size)
           }
        '''))

        #                            _                   _       _
        #   ___  _ __   ___ ___   __| | ___   _ __  _ __(_)_ __ | |_ ___ _ __
        #  / _ \| '_ \ / __/ _ \ / _` |/ _ \ | '_ \| '__| | '_ \| __/ _ \ '__|
        # | (_) | |_) | (_| (_) | (_| |  __/ | |_) | |  | | | | | ||  __/ |
        #  \___/| .__/ \___\___/ \__,_|\___| | .__/|_|  |_|_| |_|\__\___|_|
        #       |_|                          |_|

        out_file.write(textwrap.dedent('''
            pub trait OpcodePrinter<'a> {
                fn print_opcode(
                    &mut self,
                    stream: &[u8]) -> Result<u8>;
            }
            pub trait OpcodePrinterFactory<'a> {
                type Output: OpcodePrinter<'a>;
                fn new(&self, &'a mut io::Write) -> Self::Output;
            }
            pub struct OpcodePrinter8080<'a> {
                stream_out: &'a mut io::Write
            }
            pub struct OpcodePrinterFactory8080;
            impl<'a> OpcodePrinterFactory<'a> for OpcodePrinterFactory8080 {
                type Output = OpcodePrinter8080<'a>;
                fn new(&self,
                    stream_out: &'a mut io::Write) -> OpcodePrinter8080<'a>
                {
                    return OpcodePrinter8080 {
                        stream_out: stream_out
                    };
                }
            }
            impl<'a> OpcodePrinter<'a> for OpcodePrinter8080<'a> {
                fn print_opcode(
                    &mut self,
                    stream: &[u8]) -> Result<u8>
                {
                    Ok(try!(dispatch_opcode(stream, self)))
                }
            }
            impl<'a> InstructionSet8080 for OpcodePrinter8080<'a> {
        '''))

        for name, args in functions.iteritems():
            out_file.write('    '
                'fn instruction_{}({}) -> Result<()>\n    {{\n'.format(
                    name.lower(),
                    ', '.join(['&mut self'] + name_args(args))))
            if name == 'not_implemented':
                name = '-'
            out_file.write('        try!(write!(self.stream_out, '
                '"{{:04}}", "{}"));\n'.format(name))
            for arg, arg_decl in zip(args, name_args(args)):
                arg_type = arg[0]
                arg_name = arg_decl.split(':')[0]
                if arg_type == 'data':
                    out_file.write('        try!(write!(self.stream_out, '
                        '" #${{:02x}}", {}));\n'.format(arg_name))
                elif arg_type == 'implicit_data':
                    out_file.write('        try!(write!(self.stream_out, '
                        '" {{}}", {}));\n'.format(arg_name))
                elif arg_type == 'address':
                    out_file.write('        try!(write!(self.stream_out, '
                        '" ${{:02x}}", {}));\n'.format(arg_name))
                else:
                    out_file.write('        try!(write!(self.stream_out, '
                        '" {{:?}}", {}));\n'.format(arg_name))
            out_file.write('        Ok(())\n')
            out_file.write('    }\n')
        out_file.write('}')

if __name__ == "__main__":
    main()
