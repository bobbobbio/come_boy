#!/usr/bin/env python

import json
import textwrap
import os

INSTRUCTION_SET_NAME = '8080'

def read_args(args, stream):
    '''
    Translate an array of description of arguments (adr, D8, D16, A, M) into a
    function call to read that argument out of stream, or appropriate literal.
    '''
    r_args = []
    for arg in args:
        if arg == 'adr':
            r_args.append(('read_u16({}).ok().expect("")').format(stream))
        elif arg.startswith('D') and arg != 'D':
            r_args.append(
                'read_u{}({}).ok().expect("")'.format(arg[1:], stream))
        elif all([c.isdigit() for c in arg]):
            r_args.append('{} as u8'.format(arg))
        else:
            assert all([c.isalpha() for c in arg])
            r_args.append('Register{}::{}'.format(INSTRUCTION_SET_NAME, arg))
    return r_args

def name_args(args):
    return ['{}{}: {}'.format(arg[0], num + 1, arg[1])
        for num, arg in enumerate(args)]

def generate(opcode_dict, out_file):
    out_file.write(textwrap.dedent('''
        use emulator_lr35902::emulator_{0}::opcodes::{{
            read_u16, read_u8, Register{0}, OpcodePrinter{0}}};

        /*
         * Warning: This file is generated.  Don't manually edit.
         * Instead edit opcodes/opcode_gen.py
         */
    '''.format(INSTRUCTION_SET_NAME)))

    #   __                  _   _               _        _     _
    #  / _|_   _ _ __   ___| |_(_) ___  _ __   | |_ __ _| |__ | | ___
    # | |_| | | | '_ \ / __| __| |/ _ \| '_ \  | __/ _` | '_ \| |/ _ \
    # |  _| |_| | | | | (__| |_| | (_) | | | | | || (_| | |_) | |  __/
    # |_|  \__,_|_| |_|\___|\__|_|\___/|_| |_|  \__\__,_|_.__/|_|\___|
    #

    functions = {}
    for info in opcode_dict.itervalues():
        instr = info['instr']
        if instr == '-':
            name = 'not_implemented'
        else:
            name = info['description'].replace(' ', '_').lower()

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
                arg_desc.append(('register',
                    'Register{}'.format(INSTRUCTION_SET_NAME)))

        if name not in functions:
            functions[name] = instr, arg_desc
        else:
            _, existing_arg_desc = functions[name]
            assert existing_arg_desc == arg_desc, \
                "{} has non consistant arugments".format(name)

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
      pub trait InstructionSet{} {{
    '''.format(INSTRUCTION_SET_NAME)))

    for name, info in functions.iteritems():
        _, args = info
        out_file.write('    '
            'fn {}({});\n'.format(
                name.lower(),
                ', '.join(['&mut self'] + name_args(args))))

    out_file.write('}\n')

    out_file.write(textwrap.dedent('''
        pub fn dispatch_{0}_opcode<I: InstructionSet{0}>(
            mut stream: &[u8],
            machine: &mut I)
        {{
            match read_u8(&mut stream).ok().expect("") {{
    '''.format(INSTRUCTION_SET_NAME)))

    for opcode, info in opcode_dict.iteritems():
        out_file.write('        {} => '.format(opcode))
        instr = info['instr']
        if instr == '-':
            name = 'not_implemented'
        else:
            name = info['description'].replace(' ', '_').lower()
        out_file.write('machine.{}({}),\n'.format(
            name, ', '.join(read_args(info['args'], '&mut stream'))))

    out_file.write(textwrap.dedent('''
               _ => panic!("Unknown opcode")
          };
       }
    '''))

    out_file.write(textwrap.dedent('''
        pub fn get_{}_opcode_size(opcode: u8) -> u8
        {{
            match opcode {{
    '''.format(INSTRUCTION_SET_NAME)))

    for opcode, info in opcode_dict.iteritems():
        out_file.write('        {} => {},\n'.format(opcode, info['size']))

    out_file.write(textwrap.dedent('''
               _ => panic!("Unknown opcode")
          }
       }
    '''))

    #                            _                   _       _
    #   ___  _ __   ___ ___   __| | ___   _ __  _ __(_)_ __ | |_ ___ _ __
    #  / _ \| '_ \ / __/ _ \ / _` |/ _ \ | '_ \| '__| | '_ \| __/ _ \ '__|
    # | (_) | |_) | (_| (_) | (_| |  __/ | |_) | |  | | | | | ||  __/ |
    #  \___/| .__/ \___\___/ \__,_|\___| | .__/|_|  |_|_| |_|\__\___|_|
    #       |_|                          |_|

    out_file.write(textwrap.dedent('''
        impl<'a> InstructionSet{0} for OpcodePrinter{0}<'a> {{
    '''.format(INSTRUCTION_SET_NAME)))

    for name, info in functions.iteritems():
        instr, args = info
        out_file.write('    '
            'fn {}({})\n    {{\n'.format(
                name.lower(), ', '.join(['&mut self'] + name_args(args))))
        def print_to_out_file(pattern, arg_name):
            out_file.write('        '
                'write!(self.stream_out, '
                '"{}", {}).ok().expect("");\n'.format(
                    pattern, arg_name))

        print_to_out_file("{:04}", '"{}"'.format(instr))

        for arg, arg_decl in zip(args, name_args(args)):
            arg_type = arg[0]
            arg_name = arg_decl.split(':')[0]
            if arg_type == 'data':
                print_to_out_file(" #${:02x}", arg_name)
            elif arg_type == 'implicit_data':
                print_to_out_file(" {}", arg_name)
            elif arg_type == 'address':
                print_to_out_file(" ${:02x}", arg_name)
            else:
                print_to_out_file(" {:?}", arg_name)
        out_file.write('    }\n')
    out_file.write('}')

def main():
    script_dir = os.path.dirname(os.path.realpath(__file__))

    with open(os.path.join(script_dir, 'opcodes.json')) as f:
        opcode_dict = json.loads(f.read())
    output_file = os.path.join(script_dir, 'opcode_gen.rs')

    with open(output_file, 'w') as out_file:
        generate(opcode_dict, out_file)

if __name__ == "__main__":
    main()
