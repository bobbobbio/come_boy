// copyright 2021 Remi Bernotavicius

pub use super::debugger::run_debugger;
use super::joypad::{PlaybackJoyPad, RecordingJoyPad};
use super::{
    game_pak::GamePak, joypad, tandem, ControllerJoyPad, GameBoyEmulator, GameBoyOps, Result,
};
use crate::io;
use crate::rendering::Renderer;
use crate::sound::{NullSoundStream, SoundStream};
use crate::storage::{OpenMode, PersistentStorage};
use alloc::{string::String, vec::Vec};

pub fn run_emulator<Storage: PersistentStorage>(
    renderer: impl Renderer,
    sound_stream: impl SoundStream,
    storage: Storage,
    game_pak: GamePak<Storage>,
    save_state: Option<Vec<u8>>,
) -> Result<()> {
    let mut ops = GameBoyOps::new(renderer, sound_stream, storage);
    ops.load_game_pak(game_pak);
    ops.plug_in_joy_pad(ControllerJoyPad::new());

    let mut e = GameBoyEmulator::new();

    if let Some(save_state) = save_state {
        e.load_state(ops.game_pak.as_mut(), &save_state[..])?;
    }

    e.run(&mut ops);

    Ok(())
}

pub fn run_in_tandem_with<Storage: PersistentStorage>(
    out: &mut impl io::Write,
    storage: Storage,
    other_emulator_key: &str,
    game_pak: GamePak<Storage>,
    pc_only: bool,
) -> Result<()> {
    log::info!("loading {:?}", &other_emulator_key);

    tandem::run(out, storage, other_emulator_key, game_pak, pc_only)
}

pub(crate) fn run_emulator_until(
    e: &mut GameBoyEmulator,
    ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
    ticks: u64,
) {
    while e.cpu.elapsed_cycles < ticks {
        if let Some(c) = e.crashed() {
            panic!("Emulator crashed: {}", c);
        }

        e.tick(ops);
    }
}

fn run_emulator_until_and_take_screenshot(
    mut e: GameBoyEmulator,
    ops: &mut GameBoyOps<impl Renderer, impl SoundStream, impl PersistentStorage>,
    ticks: u64,
    output_stream: impl io::Write,
) {
    let ticks = e.cpu.elapsed_cycles + ticks;
    run_emulator_until(&mut e, ops, ticks);
    ops.renderer.save_buffer(output_stream).unwrap();
}

pub fn run_until_and_take_screenshot<Storage: PersistentStorage + 'static>(
    renderer: impl Renderer,
    mut storage: Storage,
    game_pak: GamePak<Storage>,
    ticks: u64,
    replay_key: Option<&str>,
    output_key: &str,
) -> Result<()> {
    let output_file = storage.open(OpenMode::Write, output_key)?;

    let mut ops = GameBoyOps::new(renderer, NullSoundStream, storage);
    if let Some(replay_key) = replay_key {
        let joy_pad = PlaybackJoyPad::new(&mut ops.storage, game_pak.hash(), replay_key)?;
        ops.plug_in_joy_pad(joy_pad);
    }
    ops.load_game_pak(game_pak);

    let e = GameBoyEmulator::new();
    run_emulator_until_and_take_screenshot(e, &mut ops, ticks, output_file);
    Ok(())
}

pub fn run_and_record_replay<Storage: PersistentStorage + 'static>(
    mut storage: Storage,
    renderer: impl Renderer,
    sound_stream: impl SoundStream,
    game_pak: GamePak<Storage>,
    output_key: &str,
) -> Result<()> {
    let joy_pad =
        RecordingJoyPad::new(&mut storage, game_pak.title(), game_pak.hash(), output_key)?;

    let mut ops = GameBoyOps::new(renderer, sound_stream, storage);
    ops.plug_in_joy_pad(joy_pad);
    ops.load_game_pak(game_pak);

    let mut e = GameBoyEmulator::new();
    e.run(&mut ops);

    Ok(())
}

pub fn playback_replay<Storage: PersistentStorage + 'static>(
    mut storage: Storage,
    renderer: impl Renderer,
    sound_stream: impl SoundStream,
    game_pak: GamePak<Storage>,
    input_key: &str,
) -> Result<()> {
    let joy_pad = PlaybackJoyPad::new(&mut storage, game_pak.hash(), input_key)?;

    let mut ops = GameBoyOps::new(renderer, sound_stream, storage);
    ops.plug_in_joy_pad(joy_pad);
    ops.load_game_pak(game_pak);

    let mut e = GameBoyEmulator::new();
    e.run(&mut ops);

    Ok(())
}

pub fn print_replay(r: impl io::Read) -> Result<String> {
    Ok(joypad::replay::print(r)?)
}
