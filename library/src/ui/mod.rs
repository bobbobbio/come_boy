// Copyright 2023 Remi Bernotavicius

use crate::game_boy_emulator::Palette;
use crate::rendering::Color;
use alloc::{format, string::String};
use egui::widgets::color_picker::color_edit_button_rgb;
use egui::widgets::Hyperlink;

const GITHUB_URL: &str = "https://github.com/bobbobbio/come_boy";

pub trait EmulatorUiHandler {
    fn load_rom_from_dialog(&mut self);
    fn loaded_rom(&mut self) -> Option<&str>;
    fn palette_mut(&mut self) -> &mut Palette;
    fn meta(&mut self, name: &str) -> String;
}

fn as_rgb(
    palette: &mut Palette,
    body: impl FnOnce(&mut [f32; 3], &mut [f32; 3], &mut [f32; 3], &mut [f32; 3]),
) {
    fn to_floats(color: &Color) -> [f32; 3] {
        [
            color.r as f32 / 255.0,
            color.g as f32 / 255.0,
            color.b as f32 / 255.0,
        ]
    }
    fn from_floats(floats: [f32; 3]) -> Color {
        Color::new(
            (floats[0] * 255.0) as u8,
            (floats[1] * 255.0) as u8,
            (floats[2] * 255.0) as u8,
        )
    }

    let mut shade0 = to_floats(&palette.shade0);
    let mut shade1 = to_floats(&palette.shade1);
    let mut shade2 = to_floats(&palette.shade2);
    let mut shade3 = to_floats(&palette.shade3);
    body(&mut shade0, &mut shade1, &mut shade2, &mut shade3);
    palette.shade0 = from_floats(shade0);
    palette.shade1 = from_floats(shade1);
    palette.shade2 = from_floats(shade2);
    palette.shade3 = from_floats(shade3);
}

pub fn render_main_gui(ui: &mut egui::Ui, emulator: &mut impl EmulatorUiHandler) {
    egui::TopBottomPanel::top("options").show_inside(ui, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Load ROM").clicked() {
                emulator.load_rom_from_dialog();
            }
            if let Some(loaded_rom) = emulator.loaded_rom() {
                ui.label(format!("playing: {loaded_rom}"));
            }
        });
        ui.collapsing("pallete", |ui| {
            let palette = emulator.palette_mut();
            as_rgb(palette, |shade0, shade1, shade2, shade3| {
                ui.horizontal(|ui| {
                    ui.label("Shade 0: ");
                    color_edit_button_rgb(ui, shade0);
                });
                ui.horizontal(|ui| {
                    ui.label("Shade 1: ");
                    color_edit_button_rgb(ui, shade1);
                });
                ui.horizontal(|ui| {
                    ui.label("Shade 2: ");
                    color_edit_button_rgb(ui, shade2);
                });
                ui.horizontal(|ui| {
                    ui.label("Shade 3: ");
                    color_edit_button_rgb(ui, shade3);
                });
            });
        });
    });

    egui::TopBottomPanel::bottom("information").show_inside(ui, |ui| {
        ui.add(Hyperlink::from_label_and_url(
            "come_boy on github",
            GITHUB_URL,
        ));
        ui.horizontal(|ui| {
            let revision = emulator.meta("revision");
            ui.label("revision: ");
            ui.add(Hyperlink::from_label_and_url(
                &revision,
                format!("{GITHUB_URL}/commit/{revision}"),
            ));
        });
        ui.label(format!("built at: {}", emulator.meta("build_date")));
    });
}
