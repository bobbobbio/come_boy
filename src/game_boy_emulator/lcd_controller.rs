// Copyright 2018 Remi Bernotavicius

extern crate sdl2;

use std::iter;

use game_boy_emulator::memory_controller::{GameBoyRegister, MemoryChunk, MemoryChunkIterator};
use game_boy_emulator::{
    BACKGROUND_DISPLAY_DATA_1, BACKGROUND_DISPLAY_DATA_2, CHARACTER_DATA, CHARACTER_DATA_1,
    CHARACTER_DATA_2, OAM_DATA, UNUSABLE_MEMORY,
};
use util::Scheduler;

/*
 * Number of pixels (both horizontal and vertical) on the screen per gameboy pixel.
 */
const PIXEL_SCALE: u32 = 4;

const CHARACTER_SIZE: u8 = 8;
const CHARACTER_AREA_SIZE: u16 = 32;

#[derive(Default)]
pub struct LCDControllerRegisters {
    pub lcdc: GameBoyRegister,
    pub stat: GameBoyRegister,
    pub scy: GameBoyRegister,
    pub scx: GameBoyRegister,
    pub ly: GameBoyRegister,
    pub lyc: GameBoyRegister,
    pub dma: GameBoyRegister,
    pub bgp: GameBoyRegister,
    pub obp0: GameBoyRegister,
    pub obp1: GameBoyRegister,
    pub wy: GameBoyRegister,
    pub wx: GameBoyRegister,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LCDBGShade {
    Shade0 = 0x0,
    Shade1 = 0x1,
    Shade2 = 0x2,
    Shade3 = 0x3,
}

fn color_for_shade(shade: LCDBGShade) -> sdl2::pixels::Color {
    match shade {
        LCDBGShade::Shade0 => sdl2::pixels::Color::RGB(255, 255, 255),
        LCDBGShade::Shade1 => sdl2::pixels::Color::RGB(105, 150, 150),
        LCDBGShade::Shade2 => sdl2::pixels::Color::RGB(50, 50, 50),
        LCDBGShade::Shade3 => sdl2::pixels::Color::RGB(0, 0, 0),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LCDControlFlag {
    DisplayOn = 0b10000000,
    WindowCodeAreaSelection = 0b01000000,
    WindowingOn = 0b00100000,
    BGCharacterDataSelection = 0b00010000,
    BGCodeAreaSelection = 0b00001000,
    ObjectBlockCompositionSelection = 0b00000100,
    ObjectOn = 0b00000010,
    BGDisplayOn = 0b00000001,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LCDStatusFlag {
    InterruptLYMatching = 0b10000000,
    InterruptMode10 = 0b01000000,
    InterruptMode01 = 0b00100000,
    InterruptMode00 = 0b00010000,
    LYMatch = 0b00001000,
    Unknown = 0b00000100,
    Mode = 0b00000011,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterruptFlag {
    VerticalBlanking = 0b00000001,
    LCDSTAT = 0b00000010,
    Timer = 0b00000100,
    #[allow(dead_code)]
    Serial = 0b00001000,
    #[allow(dead_code)]
    Joypad = 0b00010000,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LCDObject {
    y_coordinate: u8,
    x_coordinate: u8,
    character_code: u8,
    flags: u8,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum LCDObjectAttributeFlag {
    DisplayPriority = 0b10000000,
    VerticalFlip = 0b01000000,
    HorizantalFlip = 0b00100000,
    Palette = 0b00010000,
}

impl LCDObject {
    #[allow(dead_code)]
    fn read_flag(&self, flag: LCDObjectAttributeFlag) -> bool {
        self.flags & flag as u8 == flag as u8
    }
}

struct LCDObjectIterator<'a> {
    chunk_iterator: iter::Peekable<MemoryChunkIterator<'a>>,
}

impl<'a> Iterator for LCDObjectIterator<'a> {
    type Item = LCDObject;

    fn next(&mut self) -> Option<LCDObject> {
        if self.chunk_iterator.peek() == None {
            return None;
        } else {
            let lcd_object = LCDObject {
                y_coordinate: self.chunk_iterator.next().unwrap(),
                x_coordinate: self.chunk_iterator.next().unwrap(),
                character_code: self.chunk_iterator.next().unwrap(),
                flags: self.chunk_iterator.next().unwrap(),
            };
            return Some(lcd_object);
        }
    }
}

impl<'a> LCDObjectIterator<'a> {
    fn new(chunk: &'a MemoryChunk) -> LCDObjectIterator<'a> {
        LCDObjectIterator {
            chunk_iterator: MemoryChunkIterator::new(chunk).peekable(),
        }
    }
}

struct LCDDotData {
    data: [LCDBGShade; 64],
}

impl LCDDotData {
    fn new() -> LCDDotData {
        LCDDotData {
            data: [LCDBGShade::Shade0; 64],
        }
    }

    fn draw_line(&self, renderer: &mut sdl2::render::Renderer, x: i32, y: i32, ly: u8) {
        assert!(ly as i32 >= y && (ly as i32) < y + CHARACTER_SIZE as i32);

        let target_line = ly as i32 - y;
        let start_pixel = (target_line * CHARACTER_SIZE as i32) as usize;
        let end_pixel = start_pixel + CHARACTER_SIZE as usize;
        let iter = self.data[start_pixel..end_pixel].iter().enumerate();
        for (offset_x, &shade) in iter {
            let rect = sdl2::rect::Rect::new(
                (x + offset_x as i32) * PIXEL_SCALE as i32,
                ly as i32 * PIXEL_SCALE as i32,
                PIXEL_SCALE,
                PIXEL_SCALE,
            );
            let color = color_for_shade(shade);
            renderer.set_draw_color(color);
            renderer.fill_rect(rect).unwrap();
        }
    }
}

#[derive(Default)]
pub struct LCDController<'a> {
    renderer: Option<sdl2::render::Renderer<'a>>,
    event_pump: Option<sdl2::EventPump>,
    pub crash_message: Option<String>,
    pub character_data: MemoryChunk,
    pub background_display_data_1: MemoryChunk,
    pub background_display_data_2: MemoryChunk,
    pub oam_data: MemoryChunk,
    pub unusable_memory: MemoryChunk,
    pub registers: LCDControllerRegisters,
    scheduler: Scheduler<LCDController<'a>>,
    enabled: bool,
}

impl<'a> LCDController<'a> {
    pub fn new() -> Self {
        LCDController {
            character_data: MemoryChunk::from_range(CHARACTER_DATA),
            background_display_data_1: MemoryChunk::from_range(BACKGROUND_DISPLAY_DATA_1),
            background_display_data_2: MemoryChunk::from_range(BACKGROUND_DISPLAY_DATA_2),
            oam_data: MemoryChunk::from_range(OAM_DATA),
            unusable_memory: MemoryChunk::from_range(UNUSABLE_MEMORY),
            enabled: true,
            ..Default::default()
        }
    }

    pub fn schedule_initial_events(&mut self, now: u64) {
        self.scheduler.schedule(now + 56 + 4, Self::mode_2);
        self.scheduler.schedule(now + 56 + 456, Self::advance_ly);
        self.scheduler.schedule(now + 98648, Self::unknown_event2);
    }

    pub fn deliver_events(&mut self, now: u64) {
        for (time, event) in self.scheduler.poll(now) {
            event(self, time);
        }
    }

    pub fn start_rendering(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        let window = video_subsystem
            .window("come boy", 160 * PIXEL_SCALE, 144 * PIXEL_SCALE)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut renderer = window.renderer().build().unwrap();
        renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        renderer.clear();
        self.update_screen();

        self.renderer = Some(renderer);
        self.event_pump = Some(event_pump);
    }

    fn read_dot_data(&self, character_code: u8) -> LCDDotData {
        let mut dot_data = LCDDotData::new();

        let location = if self.read_lcd_control_flag(LCDControlFlag::BGCharacterDataSelection) {
            CHARACTER_DATA_1.start
        } else {
            CHARACTER_DATA_2.start
        } as usize
            + character_code as usize * 16;

        let mut iter = MemoryChunkIterator::new(&self.character_data)
            .skip(location)
            .take(16)
            .peekable();

        let mut i = 0;
        while iter.peek() != None {
            let byte1: u8 = iter.next().unwrap();
            let byte2: u8 = iter.next().unwrap();
            for bit in (0..8).rev() {
                let shade_upper = ((byte1 >> bit) & 0x1) << 1;
                let shade_lower = (byte2 >> bit) & 0x1;
                dot_data.data[i] = match shade_upper | shade_lower {
                    0x0 => LCDBGShade::Shade0,
                    0x1 => LCDBGShade::Shade1,
                    0x2 => LCDBGShade::Shade2,
                    0x3 => LCDBGShade::Shade3,
                    _ => panic!(""),
                };
                i += 1;
            }
        }
        assert_eq!(i, 64);

        return dot_data;
    }

    fn read_lcd_control_flag(&self, flag: LCDControlFlag) -> bool {
        self.registers.lcdc.read_value() & flag as u8 == flag as u8
    }

    fn set_lcd_control_flag(&mut self, flag: LCDControlFlag, value: bool) {
        let old_value = self.registers.lcdc.read_value();
        if value {
            self.registers.lcdc.set_value(old_value | flag as u8);
        } else {
            self.registers.lcdc.set_value(old_value & !(flag as u8));
        }
    }

    #[allow(dead_code)]
    fn read_lcd_status_flag(&self, flag: LCDStatusFlag) -> bool {
        self.registers.stat.read_value() & flag as u8 == flag as u8
    }

    fn set_lcd_status_flag(&mut self, flag: LCDStatusFlag, value: bool) {
        // Mode is a four-value flag
        assert!(flag != LCDStatusFlag::Mode);

        let old_value = self.registers.stat.read_value();
        if value {
            self.registers.stat.set_value(old_value | flag as u8);
        } else {
            self.registers.stat.set_value(old_value & !(flag as u8));
        }
    }

    fn set_lcd_status_mode(&mut self, value: u8) {
        let stat = self.registers.stat.read_value() & !(LCDStatusFlag::Mode as u8);
        self.registers.stat.set_value(stat | value);
    }

    #[allow(dead_code)]
    fn read_lcd_status_mode(&mut self) -> u8 {
        self.registers.stat.read_value() & (LCDStatusFlag::Mode as u8)
    }

    pub fn set_state_post_bios(&mut self) {
        self.set_lcd_control_flag(LCDControlFlag::DisplayOn, true);
        self.set_lcd_control_flag(LCDControlFlag::BGCharacterDataSelection, true);
        self.set_lcd_control_flag(LCDControlFlag::BGDisplayOn, true);
        self.registers.bgp.set_value(0xFC);

        self.set_lcd_status_flag(LCDStatusFlag::InterruptLYMatching, true);
        self.set_lcd_status_flag(LCDStatusFlag::Unknown, true);
        self.set_lcd_status_mode(0x1);
        self.registers.dma.set_value(0xff);
        self.registers.obp0.set_value(0xff);
        self.registers.obp1.set_value(0xff);

        let character_ram = include_bytes!("assets/character_ram.bin");
        self.character_data
            .clone_range_from_slice(0..character_ram.len(), &character_ram[..]);

        let background_display_data = include_bytes!("assets/background_display_data.bin");

        self.background_display_data_1.clone_range_from_slice(
            0x100..0x100 + background_display_data.len(),
            &background_display_data[..],
        );

        let oam_data = include_bytes!("assets/oam_data.bin");

        self.oam_data.clone_from_slice(&oam_data[..]);
    }

    fn get_scroll_origin_relative_to_lcd(&self) -> (i32, i32) {
        let mut x = self.registers.scx.read_value() as i32 * -1;
        let mut y = self.registers.scy.read_value() as i32 * -1;

        /*
         * This supports the behavior of the background wrapping
         */
        if x < -128 {
            x += 256;
        }

        if y < -128 {
            y += 256;
        }

        return (x, y);
    }

    fn get_window_origin_relative_to_lcd(&self) -> (i32, i32) {
        let x = self.registers.wx.read_value() as i32 * -1;
        let y = self.registers.wy.read_value() as i32 * -1;

        return (x, y);
    }

    pub fn crashed(&self) -> bool {
        self.crash_message.is_some()
    }

    fn check_for_screen_close(&mut self) {
        if self.renderer.is_none() {
            return;
        }

        for event in self.event_pump.as_mut().unwrap().poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    self.crash_message = Some(String::from("Screen Closed"));
                }
                _ => {}
            }
        }
    }

    fn clear_line(&mut self, ly: u8) {
        let rect = sdl2::rect::Rect::new(
            0,
            ly as i32 * PIXEL_SCALE as i32,
            200 * PIXEL_SCALE,
            PIXEL_SCALE,
        );
        let color = color_for_shade(LCDBGShade::Shade0);
        self.renderer.as_mut().unwrap().set_draw_color(color);
        self.renderer.as_mut().unwrap().fill_rect(rect).unwrap();
    }

    fn draw_bg_data(&mut self) {
        if self.renderer.is_none() {
            return;
        }

        let ly = self.registers.ly.read_value();

        self.clear_line(ly);

        let (scroll_x, scroll_y) = self.get_scroll_origin_relative_to_lcd();

        if scroll_y > ly as i32 {
            return;
        }

        let iter = match self.read_lcd_control_flag(LCDControlFlag::BGCodeAreaSelection) {
            false => MemoryChunkIterator::new(&self.background_display_data_1),
            true => MemoryChunkIterator::new(&self.background_display_data_2),
        };

        let tile_y = (ly as i32 - scroll_y) / CHARACTER_SIZE as i32;
        let iter = iter
            .skip(tile_y as usize * CHARACTER_AREA_SIZE as usize)
            .take(CHARACTER_AREA_SIZE as usize)
            .enumerate();

        for (tile_x, character_code) in iter {
            let character_data = self.read_dot_data(character_code);
            character_data.draw_line(
                self.renderer.as_mut().unwrap(),
                scroll_x + (tile_x as i32 * CHARACTER_SIZE as i32),
                scroll_y + (tile_y as i32 * CHARACTER_SIZE as i32),
                ly,
            );
        }
    }

    #[allow(dead_code)]
    fn draw_oam_data(&mut self) {
        if self.renderer.is_none() {
            return;
        }

        let ly = self.registers.ly.read_value();

        let (window_x, window_y) = self.get_window_origin_relative_to_lcd();
        let iter = LCDObjectIterator::new(&self.oam_data);
        for object in iter {
            let character_data = self.read_dot_data(object.character_code);
            character_data.draw_line(
                self.renderer.as_mut().unwrap(),
                window_x + object.x_coordinate as i32,
                window_y + object.y_coordinate as i32,
                ly,
            );
        }
    }

    fn update_screen(&mut self) {
        match self.renderer.as_mut() {
            Some(r) => r.present(),
            None => {}
        }
    }

    // XXX remi: This should return a Result, also leaving this here because it might be useful.
    #[allow(dead_code)]
    fn save_screenshot<P: AsRef<std::path::Path>>(&mut self, path: P) {
        match self.renderer.as_mut() {
            Some(r) => {
                let mut pixels = r
                    .read_pixels(None, sdl2::pixels::PixelFormatEnum::ABGR8888)
                    .unwrap();
                let s = sdl2::surface::Surface::from_data(
                    &mut pixels,
                    160 * PIXEL_SCALE,
                    140 * PIXEL_SCALE,
                    160 * PIXEL_SCALE * 4,
                    sdl2::pixels::PixelFormatEnum::ABGR8888,
                )
                .unwrap();
                s.save_bmp(path).unwrap();
            }
            None => {}
        }
    }

    // The LCD modes happen like this:
    // ---------> time ----->
    // 2 33 000 2 33 000 111111111111... 2 33 000
    // .--456--.         .---4560------.
    //
    // 2 .--84--. 3 .--176--. 0 .--196--. 2 ...
    //
    // The first pattern 2 33 000 repeats every 456 cycles
    // Mode 1 lasts for 4560 cycles.

    pub fn mode_2(&mut self, time: u64) {
        self.oam_data.borrow();
        self.unusable_memory.borrow();
        self.set_lcd_status_mode(0x2);
        self.scheduler.schedule(time + 77, Self::mode_3);
    }

    fn mode_3(&mut self, time: u64) {
        self.character_data.borrow();
        self.background_display_data_1.borrow();
        self.background_display_data_2.borrow();
        self.draw_bg_data();
        self.set_lcd_status_mode(0x3);
        self.scheduler.schedule(time + 175, Self::mode_0);
    }

    fn mode_0(&mut self, time: u64) {
        self.character_data.release();
        self.background_display_data_1.release();
        self.background_display_data_2.release();
        self.oam_data.release();
        self.unusable_memory.release();

        self.set_lcd_status_mode(0x0);

        if self.registers.ly.read_value() < 143 {
            self.scheduler.schedule(time + 204, Self::mode_2);
        } else {
            self.scheduler.schedule(time + 204, Self::mode_1);
        }
    }

    pub fn advance_ly(&mut self, time: u64) {
        // This advances the ly register, which represents the horizontal line that is currently
        // being drawn on the LCD.
        self.registers.ly.add(1);

        // There are only 154 lines, so wrap back to zero after that.
        if self.registers.ly.read_value() > 153 {
            self.registers.ly.set_value(0);
        }

        // If we are drawing the last line, it only takes 8 cycles, otherwise it takes 456.
        if self.registers.ly.read_value() == 153 {
            self.scheduler.schedule(time + 8, Self::advance_ly);
            self.scheduler.schedule(time + 12, Self::unknown_event);
        } else if self.registers.ly.read_value() == 0 {
            self.scheduler.schedule(time + 904, Self::advance_ly);
        } else {
            self.scheduler.schedule(time + 456, Self::advance_ly);
        }

        self.set_lcd_status_flag(LCDStatusFlag::Unknown, false);

        if self.registers.ly.read_value() < 144 && self.registers.ly.read_value() > 0 {
            self.oam_data.borrow();
            self.unusable_memory.borrow();
        }
    }

    pub fn unknown_event(&mut self, _time: u64) {
        self.set_lcd_status_flag(LCDStatusFlag::Unknown, true);
    }

    pub fn unknown_event2(&mut self, _time: u64) {
        self.set_lcd_status_flag(LCDStatusFlag::InterruptLYMatching, true);
    }

    fn mode_1(&mut self, time: u64) {
        self.set_lcd_status_mode(0x1);
        self.update_screen();

        self.check_for_screen_close();

        self.scheduler.schedule(time + 4552, Self::after_mode_1);
    }

    fn after_mode_1(&mut self, time: u64) {
        self.set_lcd_status_mode(0x0);
        self.oam_data.borrow();
        self.unusable_memory.borrow();

        self.scheduler.schedule(time + 8, Self::mode_2);
    }

    fn enable(&mut self) {
        assert!(!self.enabled);

        self.enabled = true;
    }

    fn disable(&mut self) {
        assert!(self.enabled);

        self.set_lcd_status_mode(0x0);
        self.registers.ly.set_value(0);
        self.scheduler.drop_events();

        self.enabled = false;
    }

    fn check_enabled_state(&mut self) {
        let lcdc_enabled = self.read_lcd_control_flag(LCDControlFlag::DisplayOn);
        if self.enabled != lcdc_enabled {
            if lcdc_enabled {
                self.enable();
            } else {
                self.disable();
            }
        }
    }

    pub fn tick(&mut self) {
        self.check_enabled_state();
    }
}
