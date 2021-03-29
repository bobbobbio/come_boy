# come_boy

Come boy is a gameboy emulator (DMG only) written in rust.

Written by Remi Bernotavicius (remi@abort.cc)

Currently in a pre-release state.  Runs some games, but many others do not work.

![tetris](/test/expectations/tetris/10000000.bmp?raw=true "Tetris")
![kirby](/test/expectations/kirby_dream_land/50000000_replay1.bmp?raw=true "Kirby")
![zelda](/test/expectations/zelda/200000000_replay1.bmp?raw=true "Zelda")
![pokemon_red](/test/expectations/pokemon_red/100000000.bmp?raw=true "Pokemon Red")
![f1race](/test/expectations/f1race/80000000.bmp?raw=true "F-1 Race")

# Building
Build like as follows.

    cargo build

If you want a usable emulator, pass the `--release` flag, as the debug build is
too slow.

The project has two rendering back-ends. `sdl2` and `speedy2d`. You can select
which ones you want via `--features`. The `speedy2d` back-end is included by
default.

# Running Tests
Run tests as follow. You only need to run `./download_test_roms` once.

    ./download_test_roms
    cargo test

# Usage

## Emulator
```
Game Boy (DMG) emulator

USAGE:
    come_boy [OPTIONS] <rom>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --renderer <renderer>         [default: default]
        --save-state <save_state>
        --scale <scale>               [default: 4]

ARGS:
    <rom>
```

Limited joypad support available. Use arrow keys + z, x, tab, enter.

Limited save-state functionality. F2 to save a state, and F3 to load a state.
Save states are stored in the current-working-directory as `save_state.bin`

SRAM (GamePak save-data) is stored in a file with a `.sav` extension in the same
directory as the passed ROM file.

There is little UI right now. If you are running it on Windows, you can
drag-and-drop a ROM file onto the emulator `.exe`. For Linux or OS X you have to
pass the path to the ROM via the command-line

## Tools

Here is a list and descriptions of other binaries that are included

### Disassembler

`cargo run --bin disassembler`
`disassembler [FLAGS] [OPTIONS] <rom>`

This tool reads instructions and data from ROM files and prints them to the
console as assembly. It has a few modes to select using the `--instruction-set`
flag.

The following are valid instruction-set values.

- `INTEL8080`; Intel 8080 Disassembler. The project includes an Intel 8080
emulator which the LR35902 emulator is built on top of, and this was useful for
development purposes.

- `LR35902`; LR35902 (GameBoy CPU) disassembler. This will disassemble a ROM as
if it only contains instructions.

- `GAMEBOY` (the default); The GameBoy disassembler. This will disassemble
  GameBoy ROMs into [RGBDS](https://rgbds.gbdev.io/). (It isn't fully
implemented for every kind of GamePak yet, so millage may vary at the moment)

The following are valid options.

- `--hide-opcodes`; If specified, the address and opcode for each line will be
  omitted in the output.

### Debugger

`cargo run --bin debugger`
`debugger [OPTIONS] <rom>`

This runs the emulator with debugging features accessible via the command line.

The debugger has the following commands

- `backtrace`; Print the current call stack.
- `break <address>`; Stop emulator when the program-counter reaches the given
  address
- `disassemble`; Disassemble set of instructions around the current
  program-counter.
- `exit`; Terminates the debugger and emulator, ending the process.
- `logging enable`; Print emulator state every instruction (warning slow)
- `logging disable`; Stop printing emulator state every instruction
- `next`; Run the emulator for one instruction
- `run`; Run the emulator.
- `set pc <address>`; Set the program-counter to the given address
- `state`; Print the current state of the emulator.
- `watch <address>`; Stop execution of the emulator immediately before the given
  address will be written to.
- `x <address>`; Print value stored in memory at given address.

### GamePak

`cargo run --bin game_pak`
`game_pak <rom>`

Prints out metadata for a given GameBoy ROM.


### Replay

`cargo run --bin replay`

Tool to record and playback GameBoy emulator gameplay. Has the following
sub-commands.

- `record <rom> --output <output> --scale <scale>`; Runs the emulator and
  records player joypad input to the given output path.

- `playback <rom> --input <input> --scale <scale>`; Runs the emulator and
  replays the joypad inputs recorded in the given input file.

- `print --input <input>`; Prints to the console a text representation of a
  replay file created with the `record` sub-command.

### Screenshot

`cargo run --bin screenshot`
`screenshot [OPTIONS] <rom> --output <output> --ticks <ticks>`

Runs the emulator with the given ROM until the CPU clock has reached the given
value of `<ticks>`, and saves the current screen to the given file.

This tool is useful for debugging since you can a consistent view of the screen
at some specific point in the middle of emulation.

The following are valid options.

- `--replay <replay>`; Playback the given replay file (see the replay tool) as
  joypad input.

### Tandem

`cargo run --bin tandem`
`tandem [FLAGS] <emulator_trace> <rom>`

Runs the given ROM in the emulator comparing the emulator state each cycle to
the state recorded in the given `<emulator_trace>`.

This is useful for comparing the emulator's function with another emulator. The
details of the trace format is not specified anywhere. Usually an existing
emulator would need to be modified to emit the correct trace.

# License
MIT License

Copyright (c) 2016-2019 Remi Bernotavicius

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
