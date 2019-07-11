// Copyright 2018 Remi Bernotavicius

use crate::sdl2;
use std::iter;

use game_boy_emulator::joypad_register::KeyEvent;
use game_boy_emulator::memory_controller::{
    GameBoyFlags, GameBoyRegister, MemoryChunk, MemoryMappedHardware,
};
use game_boy_emulator::{
    BACKGROUND_DISPLAY_DATA_1, BACKGROUND_DISPLAY_DATA_2, CHARACTER_DATA, CHARACTER_DATA_1,
    CHARACTER_DATA_2, OAM_DATA, UNUSABLE_MEMORY,
};
use util::Scheduler;

const CHARACTER_SIZE: u8 = 8;
const CHARACTER_AREA_SIZE: u16 = 32;

#[derive(Default)]
pub struct DmaRegister {
    value: u8,
    requested: bool,
}

impl MemoryMappedHardware for DmaRegister {
    fn read_value(&self, _: u16) -> u8 {
        self.read_value()
    }

    fn set_value(&mut self, _: u16, value: u8) {
        self.set_value(value);
        self.requested = true;
    }
}

impl DmaRegister {
    pub fn take_request(&mut self) -> Option<u16> {
        if self.requested {
            self.requested = false;
            Some(self.read_value() as u16 * 0x100)
        } else {
            None
        }
    }

    pub fn read_value(&self) -> u8 {
        self.value
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }
}

#[derive(Default)]
pub struct LCDControllerRegisters {
    pub lcdc: GameBoyFlags<LCDControlFlag>,
    pub stat: GameBoyFlags<LCDStatusFlag>,
    pub scy: GameBoyRegister,
    pub scx: GameBoyRegister,
    pub ly: GameBoyRegister,
    pub lyc: GameBoyRegister,
    pub dma: DmaRegister,
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

from_u8!(
    LCDControlFlag,
    LCDStatusFlag,
    InterruptFlag,
    LCDObjectAttributeFlag
);

impl LCDObject {
    #[allow(dead_code)]
    fn read_flag(&self, flag: LCDObjectAttributeFlag) -> bool {
        self.flags & flag as u8 == flag as u8
    }
}

struct LCDObjectIterator<'a> {
    chunk_iterator: iter::Peekable<std::slice::Iter<'a, u8>>,
}

impl<'a> Iterator for LCDObjectIterator<'a> {
    type Item = LCDObject;

    fn next(&mut self) -> Option<LCDObject> {
        if self.chunk_iterator.peek() == None {
            return None;
        } else {
            let lcd_object = LCDObject {
                y_coordinate: *self.chunk_iterator.next().unwrap(),
                x_coordinate: *self.chunk_iterator.next().unwrap(),
                character_code: *self.chunk_iterator.next().unwrap(),
                flags: *self.chunk_iterator.next().unwrap(),
            };
            return Some(lcd_object);
        }
    }
}

impl<'a> LCDObjectIterator<'a> {
    fn new(chunk: &'a MemoryChunk) -> LCDObjectIterator<'a> {
        LCDObjectIterator {
            chunk_iterator: chunk.as_slice().iter().peekable(),
        }
    }
}

struct LCDDotData {
    data: [LCDBGShade; 64],
    pixel_scale: u32,
}

impl LCDDotData {
    fn new(pixel_scale: u32) -> LCDDotData {
        LCDDotData {
            data: [LCDBGShade::Shade0; 64],
            pixel_scale,
        }
    }

    fn draw_line(&self, canvas: &mut sdl2::render::WindowCanvas, x: i32, y: i32, ly: u8) {
        assert!(ly as i32 >= y && (ly as i32) < y + CHARACTER_SIZE as i32);

        let target_line = ly as i32 - y;
        let start_pixel = (target_line * CHARACTER_SIZE as i32) as usize;
        let end_pixel = start_pixel + CHARACTER_SIZE as usize;
        let iter = self.data[start_pixel..end_pixel].iter().enumerate();
        for (offset_x, &shade) in iter {
            let rect = sdl2::rect::Rect::new(
                (x + offset_x as i32) * self.pixel_scale as i32,
                ly as i32 * self.pixel_scale as i32,
                self.pixel_scale,
                self.pixel_scale,
            );
            let color = color_for_shade(shade);
            canvas.set_draw_color(color);
            canvas.fill_rect(rect).unwrap();
        }
    }
}

#[derive(Default)]
pub struct LCDController<'a> {
    canvas: Option<sdl2::render::WindowCanvas>,
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
    interrupt_requested: bool,
    pixel_scale: u32,
}

impl<'a> LCDController<'a> {
    pub fn new(pixel_scale: u32) -> Self {
        LCDController {
            character_data: MemoryChunk::from_range(CHARACTER_DATA),
            background_display_data_1: MemoryChunk::from_range(BACKGROUND_DISPLAY_DATA_1),
            background_display_data_2: MemoryChunk::from_range(BACKGROUND_DISPLAY_DATA_2),
            oam_data: MemoryChunk::from_range(OAM_DATA),
            unusable_memory: MemoryChunk::from_range(UNUSABLE_MEMORY),
            enabled: true,
            pixel_scale,
            ..Default::default()
        }
    }

    pub fn schedule_initial_events(&mut self, now: u64) {
        self.scheduler.schedule(now + 56 + 4, Self::mode_2);
        self.scheduler.schedule(now + 56 + 456, Self::advance_ly);
        self.scheduler.schedule(now + 98648, Self::unknown_event2);
    }

    pub fn schedule_interrupts(&mut self, interrupt_flag: &mut GameBoyFlags<InterruptFlag>) {
        if self.interrupt_requested {
            interrupt_flag.set_flag(InterruptFlag::VerticalBlanking, true);
            self.interrupt_requested = false;
        }
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
            .window("come boy", 160 * self.pixel_scale, 144 * self.pixel_scale)
            .position_centered()
            .allow_highdpi()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        canvas.clear();
        self.update_screen();

        self.canvas = Some(canvas);
        self.event_pump = Some(event_pump);
    }

    fn read_dot_data(&self, character_code: u8) -> LCDDotData {
        let mut dot_data = LCDDotData::new(self.pixel_scale);

        let bg_character_data_selection = self
            .registers
            .lcdc
            .read_flag(LCDControlFlag::BGCharacterDataSelection);
        let location = if bg_character_data_selection {
            CHARACTER_DATA_1.start
        } else {
            CHARACTER_DATA_2.start
        } as usize
            + character_code as usize * 16;

        let mut iter = self.character_data.as_slice()[location..]
            .iter()
            .take(16)
            .peekable();

        let mut i = 0;
        while iter.peek() != None {
            let byte1: u8 = *iter.next().unwrap();
            let byte2: u8 = *iter.next().unwrap();
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

    pub fn set_state_post_bios(&mut self) {
        self.registers
            .lcdc
            .set_flag(LCDControlFlag::DisplayOn, true);
        self.registers
            .lcdc
            .set_flag(LCDControlFlag::BGCharacterDataSelection, true);
        self.registers
            .lcdc
            .set_flag(LCDControlFlag::BGDisplayOn, true);
        self.registers.bgp.set_value(0xFC);

        self.registers
            .stat
            .set_flag(LCDStatusFlag::InterruptLYMatching, true);
        self.registers.stat.set_flag(LCDStatusFlag::Unknown, true);
        self.registers.stat.set_flag_value(LCDStatusFlag::Mode, 0x1);
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

    pub fn poll_renderer(&mut self) -> Vec<KeyEvent> {
        if self.canvas.is_none() {
            return vec![];
        }

        let mut key_events = vec![];

        for event in self.event_pump.as_mut().unwrap().poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    self.crash_message = Some(String::from("Screen Closed"));
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(code),
                    ..
                } => {
                    key_events.push(KeyEvent::Down(code));
                }
                sdl2::event::Event::KeyUp {
                    keycode: Some(code),
                    ..
                } => {
                    key_events.push(KeyEvent::Up(code));
                }
                _ => {}
            }
        }

        key_events
    }

    fn draw_bg_data(&mut self) {
        if self.canvas.is_none() {
            return;
        }

        let ly = self.registers.ly.read_value();

        let (scroll_x, scroll_y) = self.get_scroll_origin_relative_to_lcd();

        if scroll_y > ly as i32 {
            return;
        }

        let bg_code_area_selection = self
            .registers
            .lcdc
            .read_flag(LCDControlFlag::BGCodeAreaSelection);

        let iter = match bg_code_area_selection {
            false => self.background_display_data_1.as_slice().iter(),
            true => self.background_display_data_2.as_slice().iter(),
        };

        let tile_y = (ly as i32 - scroll_y) / CHARACTER_SIZE as i32;
        let iter = iter
            .skip(tile_y as usize * CHARACTER_AREA_SIZE as usize)
            .take(CHARACTER_AREA_SIZE as usize)
            .enumerate();

        for (tile_x, character_code) in iter {
            let character_data = self.read_dot_data(*character_code);
            character_data.draw_line(
                self.canvas.as_mut().unwrap(),
                scroll_x + (tile_x as i32 * CHARACTER_SIZE as i32),
                scroll_y + (tile_y as i32 * CHARACTER_SIZE as i32),
                ly,
            );
        }
    }

    fn draw_oam_data(&mut self) {
        if self.canvas.is_none() {
            return;
        }

        if !self.registers.lcdc.read_flag(LCDControlFlag::ObjectOn) {
            return;
        }

        let ly = self.registers.ly.read_value();

        let (window_x, window_y) = self.get_window_origin_relative_to_lcd();
        let iter = LCDObjectIterator::new(&self.oam_data);
        for object in iter {
            let x = window_x + object.x_coordinate as i32 - 8;
            let y = window_y + object.y_coordinate as i32 - 16;
            if ly as i32 >= y && (ly as i32) < y + CHARACTER_SIZE as i32 {
                let character_data = self.read_dot_data(object.character_code);
                character_data.draw_line(self.canvas.as_mut().unwrap(), x, y, ly);
            }
        }
    }

    fn update_screen(&mut self) {
        match self.canvas.as_mut() {
            Some(r) => r.present(),
            None => {}
        }
    }

    // XXX remi: This should return a Result, also leaving this here because it might be useful.
    #[allow(dead_code)]
    fn save_screenshot<P: AsRef<std::path::Path>>(&mut self, path: P) {
        match self.canvas.as_mut() {
            Some(r) => {
                let mut pixels = r
                    .read_pixels(None, sdl2::pixels::PixelFormatEnum::ABGR8888)
                    .unwrap();
                let s = sdl2::surface::Surface::from_data(
                    &mut pixels,
                    160 * self.pixel_scale,
                    140 * self.pixel_scale,
                    160 * self.pixel_scale * 4,
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
        self.registers.stat.set_flag_value(LCDStatusFlag::Mode, 0x2);
        self.scheduler.schedule(time + 77, Self::mode_3);
    }

    fn mode_3(&mut self, time: u64) {
        self.character_data.borrow();
        self.background_display_data_1.borrow();
        self.background_display_data_2.borrow();
        self.draw_bg_data();
        self.draw_oam_data();
        self.registers.stat.set_flag_value(LCDStatusFlag::Mode, 0x3);
        self.scheduler.schedule(time + 175, Self::mode_0);
    }

    fn mode_0(&mut self, time: u64) {
        self.character_data.release();
        self.background_display_data_1.release();
        self.background_display_data_2.release();
        self.oam_data.release();
        self.unusable_memory.release();

        self.registers.stat.set_flag_value(LCDStatusFlag::Mode, 0x0);

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

        self.registers.stat.set_flag(LCDStatusFlag::Unknown, false);

        if self.registers.ly.read_value() < 144 && self.registers.ly.read_value() > 0 {
            self.oam_data.borrow();
            self.unusable_memory.borrow();
        }
    }

    pub fn unknown_event(&mut self, _time: u64) {
        self.registers.stat.set_flag(LCDStatusFlag::Unknown, true);
    }

    pub fn unknown_event2(&mut self, _time: u64) {
        self.registers
            .stat
            .set_flag(LCDStatusFlag::InterruptLYMatching, true);
    }

    fn mode_1(&mut self, time: u64) {
        self.registers.stat.set_flag_value(LCDStatusFlag::Mode, 0x1);
        self.update_screen();
        self.interrupt_requested = true;

        self.scheduler.schedule(time + 4552, Self::after_mode_1);
    }

    fn after_mode_1(&mut self, time: u64) {
        self.registers.stat.set_flag_value(LCDStatusFlag::Mode, 0x0);
        self.oam_data.borrow();
        self.unusable_memory.borrow();

        self.scheduler.schedule(time + 8, Self::mode_2);
    }

    fn enable(&mut self, time: u64) {
        assert!(!self.enabled);

        self.enabled = true;
        self.scheduler.schedule(time + 204, Self::mode_2);
        self.scheduler.schedule(time + 904, Self::advance_ly);
    }

    fn disable(&mut self) {
        assert!(self.enabled);

        self.character_data.release();
        self.background_display_data_1.release();
        self.background_display_data_2.release();
        self.oam_data.release();
        self.unusable_memory.release();

        self.registers.stat.set_flag_value(LCDStatusFlag::Mode, 0x0);
        self.registers.ly.set_value(0);
        self.scheduler.drop_events();

        self.enabled = false;
    }

    fn check_enabled_state(&mut self, time: u64) {
        let lcdc_enabled = self.registers.lcdc.read_flag(LCDControlFlag::DisplayOn);
        if self.enabled != lcdc_enabled {
            if lcdc_enabled {
                self.enable(time);
            } else {
                self.disable();
            }
        }
    }

    pub fn tick(&mut self, time: u64) {
        self.check_enabled_state(time);
    }
}
