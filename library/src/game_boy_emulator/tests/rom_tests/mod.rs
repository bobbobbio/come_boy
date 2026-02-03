// Copyright 2021 Remi Bernotavicius

use crate::game_boy_emulator::{
    joypad::PlaybackJoyPad, run_emulator_until, run_until_and_take_screenshot, GameBoyEmulator,
    GameBoyOps, GamePak, NullPerfObserver, Result,
};
use crate::io;
use crate::rendering::Renderer;
use crate::sound::NullSoundStream;
use crate::storage::{fs::Fs, OpenMode, PersistentStorage};
use std::path::Path;

fn run_until_and_save_reload_and_take_screenshot(
    renderer: impl Renderer,
    storage: impl PersistentStorage + 'static,
    rom_key: &str,
    ticks: u64,
    replay_key: Option<&str>,
    output_key: &str,
) -> Result<()> {
    use std::io::{Seek, SeekFrom};

    let mut ops = GameBoyOps::new(renderer, NullSoundStream, storage);
    let game_pak = GamePak::from_storage_without_sav(&mut ops.storage, rom_key)?;

    if let Some(replay_key) = replay_key {
        let joy_pad = PlaybackJoyPad::new(&mut ops.storage, game_pak.hash(), replay_key)?;
        ops.plug_in_joy_pad(joy_pad);
    }

    let mut e = GameBoyEmulator::new();

    ops.load_game_pak(game_pak);

    // Run the emulator some amount of time less than requested
    let initial_ticks = e.cpu.elapsed_cycles;
    let final_ticks = initial_ticks + ticks;
    let stopping_point = final_ticks - 500_000;
    run_emulator_until(&mut e, &mut ops, &mut NullPerfObserver, stopping_point);

    // Save a save state
    let mut tmp_output = tempfile::NamedTempFile::new()?;
    e.save_state(ops.game_pak.as_ref(), tmp_output.as_file_mut())?;
    tmp_output.seek(SeekFrom::Start(0))?;

    // Reload the emulator from the save state
    let game_pak = GamePak::from_storage_without_sav(&mut ops.storage, rom_key)?;
    ops.load_game_pak(game_pak);

    let mut e = GameBoyEmulator::new();
    e.load_state(ops.game_pak.as_mut(), tmp_output)?;

    // Run it the rest of the time
    run_emulator_until(&mut e, &mut ops, &mut NullPerfObserver, final_ticks);
    let mut file = ops.storage.open(OpenMode::Write, output_key)?;
    ops.renderer.save_buffer(&mut file).unwrap();

    Ok(())
}

fn diff_bmp<P1: AsRef<std::path::Path>, P2: AsRef<std::path::Path>>(
    path1: P1,
    path2: P2,
) -> Result<bool> {
    let image1 = bmp::open(path1).map_err(io::Error::other)?;
    let image2 = bmp::open(path2).map_err(io::Error::other)?;

    Ok(image1 != image2)
}

fn run_and_take_screenshot(
    rom_path: &Path,
    ticks: u64,
    replay: Option<&Path>,
    output: &Path,
) -> Result<()> {
    let mut fs = Fs::new(rom_path.parent());
    let renderer = crate::rendering::bitmap::BitmapRenderer::new(Default::default());
    let rom_key = Fs::path_to_key(rom_path)?;
    let game_pak = GamePak::from_storage_without_sav(&mut fs, &rom_key)?;
    let replay_key = if let Some(replay) = replay {
        Some(Fs::path_to_key(replay)?)
    } else {
        None
    };
    let output_key = Fs::path_to_key(output)?;
    run_until_and_take_screenshot(
        renderer,
        fs,
        game_pak,
        ticks,
        replay_key.as_deref(),
        &output_key,
    )
    .unwrap();
    Ok(())
}

fn run_and_save_reload_and_take_screenshot(
    rom_path: &Path,
    ticks: u64,
    replay: Option<&Path>,
    output: &std::path::Path,
) -> Result<()> {
    let renderer = crate::rendering::bitmap::BitmapRenderer::new(Default::default());
    let fs = Fs::new(rom_path.parent());
    let rom_key = Fs::path_to_key(rom_path)?;
    let output_key = Fs::path_to_key(output)?;
    let replay_key = if let Some(replay) = replay {
        Some(Fs::path_to_key(replay)?)
    } else {
        None
    };
    run_until_and_save_reload_and_take_screenshot(
        renderer,
        fs,
        &rom_key,
        ticks,
        replay_key.as_deref(),
        &output_key,
    )
    .unwrap();
    Ok(())
}

fn compare_screenshots(expectation_path: &str, actual: &std::path::Path) -> Result<()> {
    let difference = if std::env::var("ROM_TEST_UPDATE_EXPECTATION").is_ok() {
        std::fs::copy(actual, expectation_path)?;
        false
    } else {
        diff_bmp(actual, expectation_path)?
    };
    if difference {
        let failure_image: std::path::PathBuf = std::env::var("OUT_DIR").unwrap().into();
        let failure_image = failure_image.join(expectation_path);
        std::fs::create_dir_all(failure_image.parent().unwrap())?;
        std::fs::rename(actual, &failure_image)?;
        panic!(
            "Failure. Image {} does not match expectation {}",
            failure_image.to_string_lossy(),
            expectation_path
        );
    } else {
        println!("Success, images match");
    }
    Ok(())
}

#[allow(dead_code)]
fn do_rom_test(
    rom_path: &str,
    ticks: u64,
    expectation_path: &str,
    replay: Option<&str>,
) -> Result<()> {
    println!(
        "Doing ROM test for {} until clock offset {}, with replay {:?}",
        rom_path, ticks, replay
    );

    let tmp_output = tempfile::NamedTempFile::new()?;
    run_and_take_screenshot(
        rom_path.as_ref(),
        ticks,
        replay.map(|p| p.as_ref()),
        tmp_output.path(),
    )?;
    compare_screenshots(expectation_path, tmp_output.path())?;
    Ok(())
}

#[allow(dead_code)]
fn do_save_state_rom_test(
    rom_path: &str,
    ticks: u64,
    expectation_path: &str,
    replay: Option<&str>,
) -> Result<()> {
    println!(
        "Doing save-state ROM test for {} until clock offset {}, with replay {:?}",
        rom_path, ticks, replay
    );

    let tmp_output = tempfile::NamedTempFile::new()?;
    run_and_save_reload_and_take_screenshot(
        rom_path.as_ref(),
        ticks,
        replay.map(|p| p.as_ref()),
        tmp_output.path(),
    )?;
    compare_screenshots(expectation_path, tmp_output.path())?;
    Ok(())
}

mod gen;
