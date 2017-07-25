# Copyright 2017 Remi Bernotavicius

import json
import textwrap
import os

SRC_PATH = os.path.dirname(os.path.realpath(__file__))
while os.path.basename(SRC_PATH) != 'src':
    SRC_PATH = os.path.dirname(SRC_PATH)

class ArgumentType(object):
    def __init__(self):
        self.var_name = None
        self.type = None

    def arg_name(self, num):
        return "{}{}".format(self.var_name, num)

    def format(self, num):
        return "{}: {}".format(self.arg_name(num), self.type)

    def fmt_representation(self):
        return str(self)

    def make_expression(self, _stream, _value):
        return str(self)

    def __eq__(self, other):
        return self.__class__ == other.__class__ and \
               self.var_name == other.var_name and \
               self.type == other.type

    def __neq__(self, other):
        return self != other

class AddressArgumentType(ArgumentType):
    def __init__(self):
        super(AddressArgumentType, self).__init__()
        self.var_name = 'address'
        self.type = 'u16'

    def fmt_representation(self):
        return "${:02x}"

    def make_expression(self, stream, _value):
        return 'read_u16({}).unwrap()'.format(stream)

class DataArgumentType(ArgumentType):
    def __init__(self, size):
        super(DataArgumentType, self).__init__()
        self.var_name = 'data'
        self.size = size
        self.type = 'u' + str(size)

    def fmt_representation(self):
        return "#${:02x}"

    def make_expression(self, stream, _value):
        return 'read_u{}({}).unwrap()'.format(self.size, stream)

class ImplicitDataArgumentType(ArgumentType):
    def __init__(self):
        super(ImplicitDataArgumentType, self).__init__()
        self.var_name = 'implicit_data'
        self.type = 'u8'

    def fmt_representation(self):
        return "{}"

    def make_expression(self, _stream, value):
        return '{} as u8'.format(value)

class RegisterArgumentType(ArgumentType):
    def __init__(self):
        super(RegisterArgumentType, self).__init__()
        self.var_name = 'register'
        self.type = 'Intel8080Register'

    def fmt_representation(self):
        return "{:?}"

    def make_expression(self, _stream, value):
        return '{}::{}'.format(self.type, value)

class Argument(object):
    def __init__(self, arg_type, value):
        self.value = value
        self.type = arg_type

    def make_expression(self, stream):
        return self.type.make_expression(stream, self.value)

def argument_factory(arg):
    if arg == 'adr':
        return Argument(AddressArgumentType(), None)
    elif arg.startswith('D') and arg != 'D':
        return Argument(DataArgumentType(int(arg[1:])), None)
    elif all([c.isdigit() for c in arg]):
        return Argument(ImplicitDataArgumentType(), int(arg))
    else:
        assert all([c.isalpha() for c in arg])
        return Argument(RegisterArgumentType(), arg)

class OpcodeFunction(object):
    def __init__(self, name, shorthand, arguments):
        self.name = name
        self.shorthand = shorthand
        self.arguments = arguments

    def __eq__(self, other):
        return self.__class__ == other.__class__ and \
               self.name == other.name and \
               self.shorthand == other.shorthand and \
               self.arguments == other.arguments

    def __neq__(self, other):
        return self != other

    def make_declaration(self):
        named_args = \
            ['&mut self'] + [a.format(n + 1)
                for n, a in enumerate(self.arguments)]
        return '    fn {}({})'.format(self.name.lower(), ', '.join(named_args))

class OpcodeFunctionCall(object):
    def __init__(self, function, arguments):
        self.function = function
        self.arguments = arguments

    def generate(self, stream):
        arguments = [a.make_expression(stream) for a in self.arguments]
        return '{}({})'.format(self.function.name, ', '.join(arguments))

class Opcode(object):
    def __init__(self, value, size, function_call):
        self.value = value
        self.size = size
        self.function_call = function_call

    def __lt__(self, other):
        return self.value < other.value

class Opcodes(object):
    def __init__(self, opcodes, functions):
        self.opcodes = opcodes
        self.functions = functions

def read_args(args, stream):
    r_args = []
    for arg in args:
        r_args.append(arg.make_expression(stream))
    return r_args

class OpcodeCodeGenerator(object):
    def __init__(
            self, out_file, opcode_dict, instruction_set_name, module_path):
        self.out_file = out_file
        self.opcode_dict = opcode_dict
        self.instruction_set_name = instruction_set_name
        self.module_path = module_path
        self.opcodes = self.create_opcodes()
        self.indent = 0

    def out(self, msg):
        self.out_file.write(self.indent * 4 * ' ')
        self.out_file.write(textwrap.dedent(msg))

    def generate_preamble(self):
        self.out('''
            use emulator_common::Intel8080Register;
            use {}::{}InstructionPrinter;
            use std::io;
            use util::{{read_u16, read_u8}};

            /*
             * Warning: This file is generated.  Don't manually edit.
             * Instead edit {}
             */
        '''.format(
            self.module_path,
            self.instruction_set_name,
            os.path.relpath(__file__, SRC_PATH)).lstrip('\n'))

    #   __                  _   _               _        _     _
    #  / _|_   _ _ __   ___| |_(_) ___  _ __   | |_ __ _| |__ | | ___
    # | |_| | | | '_ \ / __| __| |/ _ \| '_ \  | __/ _` | '_ \| |/ _ \
    # |  _| |_| | | | | (__| |_| | (_) | | | | | || (_| | |_) | |  __/
    # |_|  \__,_|_| |_|\___|\__|_|\___/|_| |_|  \__\__,_|_.__/|_|\___|
    #

    def create_opcodes(self):
        functions = {}
        opcodes = []
        for opcode, info in self.opcode_dict.iteritems():
            instr = info['instr']
            name = info['description'].replace(' ', '_').lower()

            args = []
            for arg in info['args']:
                args.append(argument_factory(arg))

            arg_types = []
            for arg in args:
                arg_types.append(arg.type)

            function = OpcodeFunction(name, instr, arg_types)
            if name not in functions:
                functions[name] = function
            else:
                existing_function = functions[name]
                assert existing_function == function, \
                    "{} has non consistent arguments".format(name)

            function_call = OpcodeFunctionCall(function, args)
            opcodes.append(Opcode(int(opcode, 16), info['size'], function_call))

        return Opcodes(opcodes, functions.values())

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

    def generate_instructions_trait(self):
        self.out('''
          pub trait {}InstructionSet {{
        '''.format(self.instruction_set_name))

        self.indent += 1
        for function in self.opcodes.functions:
            if function.shorthand != '-':
                self.out(function.make_declaration() + ';\n')
        self.indent -= 1

        self.out('}\n')

    def iterate_opcodes(self, failure=None):
        if failure is None:
            failure_case = 'v => panic!("Unknown opcode {}", v)'
        else:
            failure_case = '_ => {}'.format(failure)
        def start_two_byte(opcode):
            self.out(('0x{0:02X} => ' +
                'match (0x{0:02X} as u16) << 8 |\n')
                .format(opcode.value >> 8, failure))
            self.indent += 1
            if failure is None:
                self.out('read_u8(&mut stream).unwrap() as u16 {\n')
            else:
                self.out(('match read_u8(&mut stream) {{ ' +
                    'Ok(x) => x, _ => {} }} as u16' +
                    '{{\n').format(failure))

        def end_two_byte():
            self.out('{}\n'.format(failure_case))
            self.indent -= 1
            self.out('},\n')

        two_byte_opcode = None
        for opcode in sorted(self.opcodes.opcodes):
            if two_byte_opcode != None and two_byte_opcode != opcode.value >> 8:
                two_byte_opcode = None
                end_two_byte()
            if opcode.value > 0xFF and two_byte_opcode != opcode.value >> 8:
                two_byte_opcode = opcode.value >> 8
                start_two_byte(opcode)

            yield opcode

        if two_byte_opcode:
            end_two_byte()

        self.out('{}\n'.format(failure_case))

    def generate_instruction_dispatch(self):
        self.out('''
            pub fn dispatch_{}_instruction<I: {}InstructionSet>(
                mut stream: &[u8],
                machine: &mut I)
            {{
                let opcode = read_u8(&mut stream).unwrap();
                match opcode {{
        '''.format(
            self.instruction_set_name.lower(), self.instruction_set_name))

        self.indent += 2
        for opcode in self.iterate_opcodes():
            if opcode.function_call.function.shorthand != '-':
                self.out('0x{:02X} => machine.{},\n'.format(
                    opcode.value,
                    opcode.function_call.generate('&mut stream')))

        self.indent -= 1
        self.out('};\n')
        self.indent -= 1
        self.out('}\n')

    def generate_get_instruction(self):
        self.out('''
            pub fn get_{}_instruction<R: io::Read>(
                mut stream: R) -> Option<Vec<u8>>
            {{
                let (mut instr, size) = match read_u8(&mut stream).unwrap() {{
        '''.format(self.instruction_set_name.lower()))

        def split_bytes(value):
            bytes = []
            while value > 0:
                bytes.append('0x{:02X}'.format(value & 0xFF))
                value >>= 8;

            if bytes:
                return ', '.join(reversed(bytes))
            else:
                return '0x{:02X}'.format(value)

        self.indent += 2
        for opcode in self.iterate_opcodes(failure='return None'):
            self.out('0x{:02X} => '.format(opcode.value))
            self.out('(vec![{}], {}),\n'.format(
                split_bytes(opcode.value), opcode.size))

        self.indent -= 1
        self.out('};\n')
        self.indent -= 1

        self.out('''
                let op_size = instr.len();
                instr.resize(size, 0);
                stream.read(&mut instr[op_size..]).unwrap();
                return Some(instr);
            }\n''')

    #                            _                   _       _
    #   ___  _ __   ___ ___   __| | ___   _ __  _ __(_)_ __ | |_ ___ _ __
    #  / _ \| '_ \ / __/ _ \ / _` |/ _ \ | '_ \| '__| | '_ \| __/ _ \ '__|
    # | (_) | |_) | (_| (_) | (_| |  __/ | |_) | |  | | | | | ||  __/ |
    #  \___/| .__/ \___\___/ \__,_|\___| | .__/|_|  |_|_| |_|\__\___|_|
    #       |_|                          |_|

    def generate_opcode_printer(self):
        self.out('''
            impl<'a> {0}InstructionSet for {0}InstructionPrinter<'a> {{
        '''.format(self.instruction_set_name))

        self.indent += 1
        for function in self.opcodes.functions:
            if function.shorthand == '-':
                continue
            self.out(function.make_declaration() + '\n')
            self.out('{\n')
            self.indent += 1

            pattern = "{:04}"
            args = '"{}"'.format(function.shorthand)

            for num, arg in enumerate(function.arguments):
                pattern += ' ' + arg.fmt_representation()
                args += ', ' + arg.arg_name(num + 1)

            self.out('self.error = write!(self.stream_out, "{}", {});\n'
                .format(pattern, args))

            self.indent -= 1
            self.out('}\n')
        self.indent -= 1
        self.out('}\n')

    def generate(self):
        self.generate_preamble()
        self.generate_instructions_trait()
        self.generate_instruction_dispatch()
        self.generate_get_instruction()
        self.generate_opcode_printer()

def generate_opcode_rs(path, instruction_set_name):
    with open(os.path.join(path, 'opcodes.json')) as f:
        opcode_dict = json.loads(f.read())
    output_file = os.path.join(path, 'opcode_gen.rs')

    module_path = '::'.join(os.path.relpath(path, SRC_PATH).split('/'))

    with open(output_file, 'w') as out_file:
        generator = OpcodeCodeGenerator(
            out_file, opcode_dict, instruction_set_name, module_path)
        generator.generate()
