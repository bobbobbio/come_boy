#!/usr/bin/env python

import json
import textwrap
import os

def vec_to_str(vec):
   return '[' + ', '.join(['"{}"'.format(x) for x in vec]) + ']'

def main():
    script_dir = os.path.dirname(os.path.realpath(__file__))

    with open(os.path.join(script_dir, 'opcodes.json')) as f:
        opcode_dict = json.loads(f.read())
    output_file = os.path.join(script_dir, 'opcode_gen.rs')

    with open(output_file, 'w') as out_file:
       out_file.write(textwrap.dedent('''
          pub fn lookup_opcode(opcode: u8) ->
             (&'static str, u8, Vec<&'static str>) {
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

if __name__ == "__main__":
    main()
