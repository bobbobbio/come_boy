// copyright 2021 Remi Bernotavicius
use crate::emulator_common::debugger::{Debugger, DebuggerOps, SimulatedInstruction};
use crate::emulator_common::{disassembler::Disassembler, MemoryAccessor};
use crate::game_boy_emulator::disassembler::RGBDSInstructionPrinterFactory;
use crate::game_boy_emulator::game_pak::GamePak;
use crate::game_boy_emulator::joypad::PlainJoyPad;
use crate::game_boy_emulator::{
    GameBoyEmulator, GameBoyOps, ModuloCounter, Underclocker, SLEEP_INPUT_TICKS,
};
use crate::io::{self, Result};
use crate::lr35902_emulator::debugger::LR35902Debugger;
use crate::rendering::Renderer;
use crate::sound::SoundStream;
use crate::storage::PersistentStorage;
use alloc::{string::String, vec, vec::Vec};
use core::{fmt, str};

impl fmt::Debug for GameBoyEmulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:#?}", self.cpu)?;
        writeln!(f, "{:#?}", self.bridge.registers)?;
        writeln!(f, "{:#?}", self.bridge.timer)?;
        writeln!(f, "{:#?}", self.bridge.lcd_controller.registers)?;
        write!(f, "{:#?}", self.bridge.sound_controller)?;

        Ok(())
    }
}

struct GameBoyDebugger<Renderer, SoundStream, Storage: PersistentStorage> {
    emulator: GameBoyEmulator,
    ops: GameBoyOps<Renderer, SoundStream, Storage>,
    underclocker: Underclocker,
    sometimes: ModuloCounter,
}

impl<R: Renderer, S: SoundStream, P: PersistentStorage> GameBoyDebugger<R, S, P> {
    fn new(ops: GameBoyOps<R, S, P>) -> Self {
        let emulator = GameBoyEmulator::new();
        let underclocker = Underclocker::new(emulator.cpu.elapsed_cycles, ops.clock_speed_hz);
        Self {
            emulator,
            ops,
            underclocker,
            sometimes: ModuloCounter::new(SLEEP_INPUT_TICKS),
        }
    }
}

impl<R: Renderer, S: SoundStream, P: PersistentStorage> DebuggerOps for GameBoyDebugger<R, S, P> {
    fn read_memory(&self, address: u16) -> u8 {
        self.ops
            .memory_map(&self.emulator.bridge)
            .read_memory(address)
    }

    fn format(&self, s: &mut dyn io::Write) -> Result<()> {
        write!(s, "{:?}", &self.emulator)
    }

    fn next(&mut self) {
        self.emulator.tick(&mut self.ops);
        if self.sometimes.incr() {
            self.underclocker.underclock(self.emulator.elapsed_cycles());
            self.emulator.read_key_events(&mut self.ops).unwrap();
        }
    }

    fn simulate_next(&mut self, instruction: &mut SimulatedInstruction) {
        let mut memory_map = self.ops.memory_map(&self.emulator.bridge);
        let mut d = LR35902Debugger::new(&mut self.emulator.cpu, &mut memory_map);
        d.simulate_next(instruction);
    }

    fn read_program_counter(&self) -> u16 {
        self.emulator.cpu.read_program_counter()
    }

    fn crashed(&self) -> Option<&String> {
        self.emulator.crashed()
    }

    fn set_program_counter(&mut self, address: u16) {
        self.emulator.cpu.set_program_counter(address)
    }

    fn disassemble(&mut self, address: u16, f: &mut dyn io::Write) -> Result<()> {
        let mut buffer = vec![];
        let memory_map = self.ops.memory_map(&self.emulator.bridge);
        let mut dis = Disassembler::new(&memory_map, RGBDSInstructionPrinterFactory, &mut buffer);
        dis.index = address;
        dis.disassemble_multiple().unwrap();
        write!(f, "{}", str::from_utf8(&buffer).unwrap())
    }

    fn read_call_stack(&self) -> Vec<u16> {
        self.emulator.cpu.call_stack.clone()
    }
}

pub fn run_debugger<Storage: PersistentStorage>(
    renderer: impl Renderer,
    sound_stream: impl SoundStream,
    storage: Storage,
    game_pak: GamePak<Storage>,
    mut input: impl Iterator<Item = io::Result<String>>,
    mut output: impl io::Write,
    is_interrupted: &dyn Fn() -> bool,
) {
    let mut ops = GameBoyOps::new(renderer, sound_stream, storage);
    ops.plug_in_joy_pad(PlainJoyPad::new());
    ops.load_game_pak(game_pak);

    let mut gameboy_debugger = GameBoyDebugger::new(ops);

    let mut debugger = Debugger::new(&mut input, &mut output, &mut gameboy_debugger);
    debugger.run(is_interrupted);
}
