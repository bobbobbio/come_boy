#!/usr/bin/env python

import json
import textwrap
import os

INSTRUCTION_SET_NAME = '8080'

class Argument(object):
    def __init__(self):
        self.var_name = None
        self.type = None

    def arg_name(self, num):
        return "{}{}".format(self.var_name, num)

    def format(self, num):
        return "{}: {}".format(self.arg_name(num), self.type)

    def fmt_representation(self):
        return str(self)

    def __eq__(self, other):
        return self.var_name == other.var_name and self.type == other.type

    def __neq__(self, other):
        return self != other

class AddressArgument(Argument):
    def __init__(self):
        super(AddressArgument, self).__init__()
        self.var_name = 'address'
        self.type = 'u16'

    def fmt_representation(self):
        return "${:02x}"

class DataArgument(Argument):
    def __init__(self, size):
        super(DataArgument, self).__init__()
        self.var_name = 'data'
        self.type = 'u' + str(size)

    def fmt_representation(self):
        return "#${:02x}"

class ImplicitDataArgument(Argument):
    def __init__(self):
        super(ImplicitDataArgument, self).__init__()
        self.var_name = 'implicit_data'
        self.type = 'u8'

    def fmt_representation(self):
        return "{}"

class RegisterArgument(Argument):
    def __init__(self):
        super(RegisterArgument, self).__init__()
        self.var_name = 'register'
        self.type = 'Register{}'.format(INSTRUCTION_SET_NAME)

    def fmt_representation(self):
        return "{:?}"

def argument_factory(arg):
    if arg == 'adr':
        return AddressArgument()
    elif arg.startswith('D') and arg != 'D':
        return DataArgument(int(arg[1:]))
    elif all([c.isdigit() for c in arg]):
        return ImplicitDataArgument()
    else:
        assert all([c.isalpha() for c in arg])
        return RegisterArgument()

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
            arg_desc.append(argument_factory(arg))

        if name not in functions:
            functions[name] = instr, arg_desc
        else:
            _, existing_arg_desc = functions[name]
            assert existing_arg_desc == arg_desc, \
                "{} has non consistent arguments".format(name)

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

    def function_declaration(name, args):
        named_args = \
            ['&mut self'] + [a.format(n + 1) for n, a in enumerate(args)]
        return '    fn {}({})'.format(name, ', '.join(named_args))

    for name, info in functions.iteritems():
        _, args = info
        out_file.write(function_declaration(name.lower(), args))
        out_file.write(';\n')

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
        out_file.write(function_declaration(name.lower(), args))
        out_file.write('\n    {\n')

        def print_to_out_file(pattern, arg_name):
            out_file.write('        '
                'write!(self.stream_out, '
                '"{}", {}).ok().expect("");\n'.format(
                    pattern, arg_name))

        print_to_out_file("{:04}", '"{}"'.format(instr))

        for num, arg in enumerate(args):
            print_to_out_file(
                ' ' + arg.fmt_representation(), arg.arg_name(num + 1))
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
