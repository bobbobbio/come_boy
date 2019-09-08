# come_boy

Come boy is a gameboy emulator (DMG only) written in rust.

Written by Remi Bernotavicius (remi@abort.cc)

Currently in a pre-release state.  Runs some games, but many others do not work.

![tetris](/test/expectations/tetris/10000000.bmp?raw=true "Tetris")

# Building
Build like as follows.

    cargo build

# Running Tests
Run tests as follow. You only need to run `./download_test_roms` once.

    ./download_test_roms
    cargo test

# Usage

## Disassember
`disassember --instruction-set [8080|LR35902] [PATH-TO-ROM]`

## Debugger
`debugger [PATH-TO-ROM]`

## Emulator
`come_boy [PATH-TO-ROM]`

Limited joypad support available. Use arrow keys + z, x, tab, enter.

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
