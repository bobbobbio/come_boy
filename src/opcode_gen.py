import json
import textwrap
import os

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
        self.type = 'Register8080'

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
    def __init__(self, out_file, opcode_dict, instruction_set_name):
        self.out_file = out_file
        self.opcode_dict = opcode_dict
        self.instruction_set_name = instruction_set_name
        self.opcodes = self.create_opcodes()

    def generate_preamble(self):
        self.out_file.write(textwrap.dedent('''
            use emulator_lr35902::emulator_{0}::opcodes::{{
                read_u16, read_u8, Register{0}, OpcodePrinter{0}}};

            /*
             * Warning: This file is generated.  Don't manually edit.
             * Instead edit opcodes/opcode_gen.py
             */
        '''.format(self.instruction_set_name)))

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
            if instr == '-':
                name = 'not_implemented'
            else:
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
            opcodes.append(Opcode(opcode, info['size'], function_call))

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
        self.out_file.write(textwrap.dedent('''
          pub trait InstructionSet{} {{
        '''.format(self.instruction_set_name)))

        for function in self.opcodes.functions:
            self.out_file.write(function.make_declaration())
            self.out_file.write(';\n')

        self.out_file.write('}\n')

    def generate_opcode_dispatch(self):
        self.out_file.write(textwrap.dedent('''
            pub fn dispatch_{0}_opcode<I: InstructionSet{0}>(
                mut stream: &[u8],
                machine: &mut I)
            {{
                match read_u8(&mut stream).unwrap() {{
        '''.format(INSTRUCTION_SET_NAME)))

        for opcode in self.opcodes.opcodes:
            self.out_file.write('        {} => '.format(opcode.value))
            self.out_file.write('machine.{},\n'.format(
                opcode.function_call.generate('&mut stream')))

        self.out_file.write(textwrap.dedent('''
                   _ => panic!("Unknown opcode")
              };
           }
        '''))

    def generate_opcode_size(self):
        self.out_file.write(textwrap.dedent('''
            pub fn get_{}_opcode_size(opcode: u8) -> u8
            {{
                match opcode {{
        '''.format(INSTRUCTION_SET_NAME)))

        for opcode in self.opcodes.opcodes:
            self.out_file.write('        {} => {},\n'.format(
                opcode.value, opcode.size))

        self.out_file.write(textwrap.dedent('''
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

    def generate_opcode_printer(self):
        self.out_file.write(textwrap.dedent('''
            impl<'a> InstructionSet{0} for OpcodePrinter{0}<'a> {{
        '''.format(INSTRUCTION_SET_NAME)))

        for function in self.opcodes.functions:
            self.out_file.write(function.make_declaration())
            self.out_file.write('\n    {\n')

            def print_to_out_file(pattern, arg_name):
                self.out_file.write('        '
                    'write!(self.stream_out, "{}", {}).unwrap();\n'.format(
                        pattern, arg_name))

            print_to_out_file("{:04}", '"{}"'.format(function.shorthand))

            for num, arg in enumerate(function.arguments):
                print_to_out_file(
                    ' ' + arg.fmt_representation(), arg.arg_name(num + 1))
            self.out_file.write('    }\n')
        self.out_file.write('}')

    def generate(self):
        self.generate_preamble()
        self.generate_instructions_trait()
        self.generate_opcode_dispatch()
        self.generate_opcode_size()
        self.generate_opcode_printer()

def generate_opcode_rs(path, instruction_set_name):
    with open(os.path.join(path, 'opcodes.json')) as f:
        opcode_dict = json.loads(f.read())
    output_file = os.path.join(path, 'opcode_gen.rs')

    with open(output_file, 'w') as out_file:
        generator = OpcodeCodeGenerator(
            out_file, opcode_dict, instruction_set_name)
        generator.generate()
