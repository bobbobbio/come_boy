// copyright 2021 Remi Bernotavicius

use super::joypad::{JoyPad, PlaybackJoyPad, RecordingJoyPad};
use super::Result;
use super::{game_pak::GamePak, joypad, tandem, ControllerJoyPad, GameBoyEmulator};
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
    let mut e = GameBoyEmulator::new();
    e.load_game_pak(game_pak);

    if let Some(save_state) = save_state {
        e.load_state(&save_state[..])?;
    }

    e.run(renderer, sound_stream, storage, ControllerJoyPad::new());
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

pub(crate) fn run_emulator_until<R: Renderer>(
    e: &mut GameBoyEmulator,
    renderer: &mut R,
    ticks: u64,
) {
    while e.cpu.elapsed_cycles < ticks {
        if let Some(c) = e.crashed() {
            panic!("Emulator crashed: {}", c);
        }

        e.tick(renderer, &mut NullSoundStream);
    }
}

fn run_emulator_until_and_take_screenshot(
    mut e: GameBoyEmulator,
    renderer: &mut impl Renderer,
    joypad: Option<impl JoyPad + 'static>,
    ticks: u64,
    output_path: impl AsRef<std::path::Path>,
) {
    if let Some(joypad) = joypad {
        e.plug_in_joy_pad(joypad);
    }
    let ticks = e.cpu.elapsed_cycles + ticks;
    run_emulator_until(&mut e, renderer, ticks);
    renderer.save_buffer(output_path).unwrap();
}

pub fn run_until_and_take_screenshot(
    renderer: &mut impl Renderer,
    game_pak: GamePak,
    ticks: u64,
    replay_path: Option<impl AsRef<std::path::Path>>,
    output_path: impl AsRef<std::path::Path>,
) -> Result<()> {
    let joypad = if let Some(replay_path) = replay_path {
        Some(PlaybackJoyPad::new(game_pak.hash(), replay_path)?)
    } else {
        None
    };

    let mut e = GameBoyEmulator::new();
    e.load_game_pak(game_pak);
    run_emulator_until_and_take_screenshot(e, renderer, joypad, ticks, output_path);
    Ok(())
}

pub fn run_and_record_replay(
    renderer: &mut impl Renderer,
    sound_stream: &mut impl SoundStream,
    storage: &mut impl PersistentStorage,
    game_pak: GamePak,
    output: &std::path::Path,
) -> Result<()> {
    let joypad = RecordingJoyPad::new(game_pak.title(), game_pak.hash(), output)?;
    let mut e = GameBoyEmulator::new();
    e.load_game_pak(game_pak);

    e.run(renderer, sound_stream, storage, joypad);
    Ok(())
}

pub fn playback_replay(
    renderer: &mut impl Renderer,
    sound_stream: &mut impl SoundStream,
    game_pak: GamePak,
    input: &std::path::Path,
) -> Result<()> {
    let joypad = PlaybackJoyPad::new(game_pak.hash(), input)?;
    let mut e = GameBoyEmulator::new();
    e.load_game_pak(game_pak);

    e.run(renderer, sound_stream, &mut PanicStorage, joypad);
    Ok(())
}

pub fn print_replay(input: &std::path::Path) -> Result<()> {
    joypad::print_replay(input)?;
    Ok(())
}
