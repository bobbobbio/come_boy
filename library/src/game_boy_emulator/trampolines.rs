// copyright 2021 Remi Bernotavicius

use super::joypad::{PlaybackJoyPad, RecordingJoyPad};
use super::{
    game_pak::GamePak, joypad, tandem, ControllerJoyPad, GameBoyEmulator, GameBoyOps, Result,
};
use crate::rendering::Renderer;
use crate::sound::{NullSoundStream, SoundStream};
use crate::storage::{PanicStorage, PersistentStorage};
use core::fmt::Debug;

pub fn run_emulator(
    renderer: &mut impl Renderer,
    sound_stream: &mut impl SoundStream,
    storage: &mut impl PersistentStorage,
    game_pak: GamePak,
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

pub fn run_in_tandem_with(
    other_emulator_path: impl AsRef<std::path::Path> + Debug,
    game_pak: GamePak,
    pc_only: bool,
) -> Result<()> {
    println!("loading {:?}", &other_emulator_path);

    tandem::run(other_emulator_path, game_pak, pc_only)
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
    output_path: impl AsRef<std::path::Path>,
) {
    let ticks = e.cpu.elapsed_cycles + ticks;
    run_emulator_until(&mut e, ops, ticks);
    ops.renderer.save_buffer(output_path).unwrap();
}

pub fn run_until_and_take_screenshot(
    renderer: &mut impl Renderer,
    game_pak: GamePak,
    ticks: u64,
    replay_path: Option<impl AsRef<std::path::Path>>,
    output_path: impl AsRef<std::path::Path>,
) -> Result<()> {
    let mut ops = GameBoyOps::new(renderer, NullSoundStream, PanicStorage);

    if let Some(replay_path) = replay_path {
        ops.plug_in_joy_pad(PlaybackJoyPad::new(game_pak.hash(), replay_path)?);
    }
    ops.load_game_pak(game_pak);

    let e = GameBoyEmulator::new();
    run_emulator_until_and_take_screenshot(e, &mut ops, ticks, output_path);
    Ok(())
}

pub fn run_and_record_replay(
    renderer: &mut impl Renderer,
    sound_stream: &mut impl SoundStream,
    storage: &mut impl PersistentStorage,
    game_pak: GamePak,
    output: &std::path::Path,
) -> Result<()> {
    let mut ops = GameBoyOps::new(renderer, sound_stream, storage);
    ops.plug_in_joy_pad(RecordingJoyPad::new(
        game_pak.title(),
        game_pak.hash(),
        output,
    )?);
    ops.load_game_pak(game_pak);

    let mut e = GameBoyEmulator::new();
    e.run(&mut ops);

    Ok(())
}

pub fn playback_replay(
    renderer: &mut impl Renderer,
    sound_stream: &mut impl SoundStream,
    game_pak: GamePak,
    input: &std::path::Path,
) -> Result<()> {
    let mut ops = GameBoyOps::new(renderer, sound_stream, PanicStorage);
    ops.plug_in_joy_pad(PlaybackJoyPad::new(game_pak.hash(), input)?);
    ops.load_game_pak(game_pak);

    let mut e = GameBoyEmulator::new();
    e.run(&mut ops);

    Ok(())
}

pub fn print_replay(input: &std::path::Path) -> Result<()> {
    joypad::print_replay(input)?;
    Ok(())
}
