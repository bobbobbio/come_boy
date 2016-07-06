#!/usr/bin/env python

import json
import textwrap
import os

def vec_to_str(vec):
   return '[' + ', '.join(['"{}"'.format(x) for x in vec]) + ']'

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
            pub fn lookup_opcode(
                opcode: u8) -> (&'static str, u8, Vec<&'static str>)
            {
                match opcode {
        '''))

        for opcode, info in opcode_dict.iteritems():
          out_file.write('        '
             '{} => ("{}", {}, vec!{}),\n'.format(int(opcode, 16),
                info['instr'], info['size'], vec_to_str(info['args'])))

        out_file.write(textwrap.dedent('''
                   _ => ("unknown", 1, vec![])
              }
           }
        '''))

        functions = {}
        registers = set()
        for info in opcode_dict.itervalues():
            name = info['instr']
            if name == '-':
                continue

            arg_desc = []
            for arg in info['args']:
                if arg == 'adr':
                    arg_desc.append(('address', 'u8'))
                elif arg.startswith('D') and arg != 'D':
                    arg_desc.append(('data', 'u' + arg[1:]))
                elif all([c.isdigit() for c in arg]):
                    arg_desc.append(('data', 'u8'))
                else:
                    assert all([c.isalpha() for c in arg])
                    arg_desc.append(('register', 'Register8080'))
                    registers.add(arg)

            if name not in functions:
                functions[name] = arg_desc
            else:
                assert functions[name] == arg_desc, "{}".format(name)

        out_file.write(textwrap.dedent('''
          enum Register8080 {
        '''))
        for register_name in sorted(registers):
            out_file.write('    ' + register_name + ',\n')
        out_file.write('}\n')

        out_file.write(textwrap.dedent('''
          trait InstructionSet8080 {
        '''))

        for name, args in functions.iteritems():
            out_file.write('    '
                'fn instruction_{}({});\n'.format(
                    name.lower(),
                    ', '.join(['&self'] + name_args(args))))

        out_file.write('}')

if __name__ == "__main__":
    main()
