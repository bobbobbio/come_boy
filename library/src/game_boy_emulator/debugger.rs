use crate::emulator_common::debugger::{Debugger, DebuggerOps, SimulatedInstruction};
use crate::emulator_common::disassembler::{Disassembler, MemoryAccessor};
use crate::game_boy_emulator::disassembler::RGBDSInstructionPrinterFactory;
use crate::game_boy_emulator::game_pak::GamePak;
use crate::game_boy_emulator::joypad::PlainJoyPad;
use crate::game_boy_emulator::memory_controller::GameBoyMemoryMap;
use crate::game_boy_emulator::{GameBoyEmulator, ModuloCounter, Underclocker, SLEEP_INPUT_TICKS};
use crate::lr35902_emulator::debugger::LR35902Debugger;
use crate::rendering::Renderer;
use crate::sound::SoundStream;
use crate::storage::PersistentStorage;
use core::{fmt, str};
use std::io::{self, Result};

impl fmt::Debug for GameBoyEmulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:#?}", self.cpu)?;
        writeln!(f, "{:#?}", self.registers)?;
        writeln!(f, "{:#?}", self.timer)?;
        write!(f, "{:#?}", self.lcd_controller.registers)?;
        write!(f, "{:#?}", self.sound_controller)?;

        Ok(())
    }
}

struct GameBoyDebugger<'a, R, S, P> {
    emulator: GameBoyEmulator,
    renderer: &'a mut R,
    sound_stream: &'a mut S,
    storage: &'a mut P,
    underclocker: Underclocker,
    sometimes: ModuloCounter,
}

impl<'a, R: Renderer, S: SoundStream, P: PersistentStorage> GameBoyDebugger<'a, R, S, P> {
    fn new(renderer: &'a mut R, sound_stream: &'a mut S, storage: &'a mut P) -> Self {
        let emulator = GameBoyEmulator::new();
        let underclocker =
            Underclocker::new(emulator.cpu.elapsed_cycles, emulator.clock_speed_hz());
        Self {
            emulator,
            renderer,
            sound_stream,
            storage,
            underclocker,
            sometimes: ModuloCounter::new(SLEEP_INPUT_TICKS),
        }
    }
}

impl<'a, R: Renderer, S: SoundStream, P: PersistentStorage> DebuggerOps
    for GameBoyDebugger<'a, R, S, P>
{
    fn read_memory(&self, address: u16) -> u8 {
        let memory_map = game_boy_memory_map!(&self.emulator);
        memory_map.read_memory(address)
    }

    fn format<'b>(&self, s: &'b mut dyn io::Write) -> Result<()> {
        write!(s, "{:?}", &self.emulator)
    }

    fn next(&mut self) {
        self.emulator.tick(self.renderer, self.sound_stream);
        if self.sometimes.incr() {
            self.underclocker.underclock(self.emulator.elapsed_cycles());
            self.emulator
                .read_key_events(self.renderer, self.storage)
                .unwrap();
        }
    }

    fn simulate_next(&mut self, instruction: &mut SimulatedInstruction) {
        let mut memory_map = game_boy_memory_map!(self.emulator);
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
        let memory_map = game_boy_memory_map!(&self.emulator);
        let mut dis = Disassembler::new(&memory_map, RGBDSInstructionPrinterFactory, &mut buffer);
        dis.index = address;
        dis.disassemble_multiple().unwrap();
        write!(f, "{}", str::from_utf8(&buffer).unwrap())
    }

    fn read_call_stack(&self) -> Vec<u16> {
        self.emulator.cpu.call_stack.clone()
    }
}

pub fn run_debugger(
    renderer: &mut impl Renderer,
    sound_stream: &mut impl SoundStream,
    storage: &mut impl PersistentStorage,
    game_pak: GamePak,
    is_interrupted: &dyn Fn() -> bool,
) {
    let mut gameboy_debugger = GameBoyDebugger::new(renderer, sound_stream, storage);
    gameboy_debugger.emulator.load_game_pak(game_pak);
    gameboy_debugger
        .emulator
        .plug_in_joy_pad(PlainJoyPad::new());

    let stdin = &mut io::stdin();
    let stdin_locked = &mut stdin.lock();
    let stdout = &mut io::stdout();

    let mut debugger = Debugger::new(stdin_locked, stdout, &mut gameboy_debugger);
    debugger.run(is_interrupted);
}
