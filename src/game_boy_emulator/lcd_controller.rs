// Copyright 2018 Remi Bernotavicius

use crate::game_boy_emulator::memory_controller::{
    FlagMask, GameBoyFlags, GameBoyRegister, MemoryChunk, MemoryMappedHardware,
};
use crate::game_boy_emulator::Result;
use crate::rendering::{Color, Renderer};
use crate::util::Scheduler;
use serde_derive::{Deserialize, Serialize};
use std::iter;
use std::ops::Range;

const SCREEN_WIDTH: i32 = 160;
const SCREEN_HEIGHT: i32 = 144;

const CHARACTER_SIZE: i32 = 8;

const CHARACTER_AREA_SIZE: u16 = 32;

const CHARACTER_DATA: Range<u16> = Range {
    start: 0x8000,
    end: 0x9800,
};

const CHARACTER_DATA_1: Range<u16> = Range {
    start: 0x0,
    end: 0x1000,
};

const CHARACTER_DATA_2: Range<u16> = Range {
    start: 0x800,
    end: 0x1800,
};

const BACKGROUND_DISPLAY_DATA_1: Range<u16> = Range {
    start: 0x9800,
    end: 0x9C00,
};

const BACKGROUND_DISPLAY_DATA_2: Range<u16> = Range {
    start: 0x9C00,
    end: 0xA000,
};

pub const OAM_DATA: Range<u16> = Range {
    start: 0xFE00,
    end: 0xFEA0,
};

const UNUSABLE_MEMORY: Range<u16> = Range {
    start: 0xFEA0,
    end: 0xFF00,
};

/// The maximum number of sprites that can appear on one line
const LINE_SPRITE_LIMIT: usize = 10;

#[derive(Default, Serialize, Deserialize)]
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

#[derive(Default, Serialize, Deserialize)]
pub struct LCDControllerRegisters {
    pub lcdc: GameBoyFlags<LCDControlFlag>,
    pub stat: GameBoyFlags<LCDStatusFlag>,
    pub scy: GameBoyRegister,
    pub scx: GameBoyRegister,
    pub ly: GameBoyRegister,
    pub lyc: GameBoyRegister,
    pub dma: DmaRegister,
    pub bgp: GameBoyFlags<LCDColor>,
    pub obp0: GameBoyFlags<LCDColor>,
    pub obp1: GameBoyFlags<LCDColor>,
    pub wy: GameBoyRegister,
    pub wx: GameBoyRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LCDColor {
    Color3 = 0b11000000,
    Color2 = 0b00110000,
    Color1 = 0b00001100,
    Color0 = 0b00000011,
}

impl FlagMask for LCDColor {
    fn mask() -> u8 {
        0xFF
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LCDShade {
    Shade0 = 0x0,
    Shade1 = 0x1,
    Shade2 = 0x2,
    Shade3 = 0x3,
}

fn color_for_shade<R: Renderer>(shade: LCDShade) -> R::Color {
    match shade {
        LCDShade::Shade0 => R::Color::new(0xe0, 0xf8, 0xd0),
        LCDShade::Shade1 => R::Color::new(0x88, 0xc0, 0x70),
        LCDShade::Shade2 => R::Color::new(0x34, 0x68, 0x56),
        LCDShade::Shade3 => R::Color::new(0x08, 0x18, 0x20),
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

impl FlagMask for LCDControlFlag {
    fn mask() -> u8 {
        0xFF
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LCDStatusFlag {
    InterruptLYMatching = 0b01000000,
    InterruptMode10 = 0b00100000,
    InterruptMode01 = 0b00010000,
    InterruptMode00 = 0b00001000,
    LYMatch = 0b00000100,
    Mode = 0b00000011,
}

impl FlagMask for LCDStatusFlag {
    fn mask() -> u8 {
        0x7F
    }
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

impl FlagMask for InterruptFlag {
    fn mask() -> u8 {
        InterruptFlag::VerticalBlanking as u8
            | InterruptFlag::LCDSTAT as u8
            | InterruptFlag::Timer as u8
            | InterruptFlag::Serial as u8
            | InterruptFlag::Joypad as u8
    }
}

impl From<InterruptEnableFlag> for InterruptFlag {
    fn from(f: InterruptEnableFlag) -> Self {
        match f {
            InterruptEnableFlag::VerticalBlanking => InterruptFlag::VerticalBlanking,
            InterruptEnableFlag::LCDSTAT => InterruptFlag::LCDSTAT,
            InterruptEnableFlag::Timer => InterruptFlag::Timer,
            InterruptEnableFlag::Serial => InterruptFlag::Serial,
            InterruptEnableFlag::Joypad => InterruptFlag::Joypad,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterruptEnableFlag {
    VerticalBlanking = 0b00000001,
    LCDSTAT = 0b00000010,
    Timer = 0b00000100,
    #[allow(dead_code)]
    Serial = 0b00001000,
    #[allow(dead_code)]
    Joypad = 0b00010000,
}

impl FlagMask for InterruptEnableFlag {
    fn mask() -> u8 {
        0xFF
    }
}

impl From<InterruptFlag> for InterruptEnableFlag {
    fn from(f: InterruptFlag) -> Self {
        match f {
            InterruptFlag::VerticalBlanking => InterruptEnableFlag::VerticalBlanking,
            InterruptFlag::LCDSTAT => InterruptEnableFlag::LCDSTAT,
            InterruptFlag::Timer => InterruptEnableFlag::Timer,
            InterruptFlag::Serial => InterruptEnableFlag::Serial,
            InterruptFlag::Joypad => InterruptEnableFlag::Joypad,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LCDObject {
    y: i32,
    x: i32,
    character_code: u8,
    flags: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LCDObjectAttributeFlag {
    DisplayPriority = 0b10000000,
    VerticalFlip = 0b01000000,
    HorizantalFlip = 0b00100000,
    Palette = 0b00010000,
}

from_u8!(
    InterruptEnableFlag,
    InterruptFlag,
    LCDControlFlag,
    LCDObjectAttributeFlag,
    LCDColor,
    LCDStatusFlag
);

impl LCDObject {
    fn read_flag(&self, flag: LCDObjectAttributeFlag) -> bool {
        self.flags & flag as u8 == flag as u8
    }

    /// Returns tuple (y position, character code)
    fn get_character_data_for_line(
        &self,
        line: i32,
        object_block_composition_selection: bool,
    ) -> (i32, u8) {
        // If we are a double-height sprite
        if object_block_composition_selection {
            let first_code = self.character_code & !1;
            let second_code = self.character_code | 1;
            let vertical_flip = self.read_flag(LCDObjectAttributeFlag::VerticalFlip);

            // Figure out which sprite the line crosses for double-height sprites
            if line < self.y + CHARACTER_SIZE {
                (
                    self.y,
                    if vertical_flip {
                        second_code
                    } else {
                        first_code
                    },
                )
            } else {
                (
                    self.y + CHARACTER_SIZE,
                    if vertical_flip {
                        first_code
                    } else {
                        second_code
                    },
                )
            }
        } else {
            (self.y, self.character_code)
        }
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
                y: *self.chunk_iterator.next().unwrap() as i32 - CHARACTER_SIZE * 2,
                x: *self.chunk_iterator.next().unwrap() as i32 - CHARACTER_SIZE,
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

struct LCDDotData<'a> {
    data: &'a [u8],
}

impl<'a> LCDDotData<'a> {
    fn read_pixel(&self, offset: usize) -> LCDColor {
        let byte_offset = (offset / 8) * 2;
        let bit_offset = 7 - (offset % 8);

        let byte1 = self.data[byte_offset];
        let byte2 = self.data[byte_offset + 1];

        let shade_upper = ((byte2 >> bit_offset) & 0x1) << 1;
        let shade_lower = (byte1 >> bit_offset) & 0x1;
        match shade_upper | shade_lower {
            0x0 => LCDColor::Color0,
            0x1 => LCDColor::Color1,
            0x2 => LCDColor::Color2,
            0x3 => LCDColor::Color3,
            _ => panic!(),
        }
    }

    fn draw_line<R: Renderer>(
        &self,
        renderer: &mut R,
        x: i32,
        y: i32,
        ly: i32,
        vertical_flip: bool,
        horizantal_flip: bool,
        enable_transparency: bool,
        palette: &GameBoyFlags<LCDColor>,
    ) {
        assert!(ly >= y && ly < y + CHARACTER_SIZE);
        assert!((ly as i32) < SCREEN_HEIGHT, "drawing ly = {}", ly);

        let target_line = if vertical_flip {
            y + CHARACTER_SIZE - 1 - ly
        } else {
            ly - y
        };
        let start_pixel = (target_line * CHARACTER_SIZE) as usize;
        let end_pixel = start_pixel + CHARACTER_SIZE as usize;
        let iter = (start_pixel..end_pixel)
            .map(|i| self.read_pixel(i))
            .enumerate();
        for (mut offset_x, color) in iter {
            if horizantal_flip {
                offset_x = CHARACTER_SIZE as usize - offset_x - 1;
            }
            if x + offset_x as i32 >= SCREEN_WIDTH {
                break;
            }
            if color != LCDColor::Color0 || !enable_transparency {
                let shade = match palette.read_flag_value(color) {
                    0x0 => LCDShade::Shade0,
                    0x1 => LCDShade::Shade1,
                    0x2 => LCDShade::Shade2,
                    0x3 => LCDShade::Shade3,
                    _ => panic!(),
                };
                let color = color_for_shade::<R>(shade);
                renderer.color_pixel(x + offset_x as i32, ly as i32, color);
            }
        }
    }
}

#[derive(PartialEq)]
enum ObjectPriority {
    Background,
    Foreground,
}

#[derive(Serialize, Deserialize)]
enum LCDControllerEvent {
    AdvanceLy,
    AfterMode1,
    Mode0,
    Mode1,
    Mode2,
    Mode3,
    UpdateLyMatch,
}

impl LCDControllerEvent {
    fn deliver<R: Renderer>(self, controller: &mut LCDController, renderer: &mut R, time: u64) {
        match self {
            Self::AdvanceLy => controller.advance_ly(time),
            Self::AfterMode1 => controller.after_mode_1(time),
            Self::Mode0 => controller.mode_0(time),
            Self::Mode1 => controller.mode_1(renderer, time),
            Self::Mode2 => controller.mode_2(time),
            Self::Mode3 => controller.mode_3(renderer, time),
            Self::UpdateLyMatch => controller.update_ly_match(time),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LCDController {
    pub crash_message: Option<String>,
    pub character_data: MemoryChunk,
    pub background_display_data_1: MemoryChunk,
    pub background_display_data_2: MemoryChunk,
    pub oam_data: MemoryChunk,
    pub unusable_memory: MemoryChunk,
    pub registers: LCDControllerRegisters,
    scheduler: Scheduler<LCDControllerEvent>,
    enabled: bool,
    interrupt_requested: bool,
    #[serde(skip)]
    object_buffer: Vec<LCDObject>,
}

impl LCDController {
    pub fn new() -> Self {
        LCDController {
            character_data: MemoryChunk::from_range(CHARACTER_DATA),
            background_display_data_1: MemoryChunk::from_range(BACKGROUND_DISPLAY_DATA_1),
            background_display_data_2: MemoryChunk::from_range(BACKGROUND_DISPLAY_DATA_2),
            oam_data: MemoryChunk::from_range(OAM_DATA),
            unusable_memory: MemoryChunk::from_range(UNUSABLE_MEMORY),
            enabled: true,
            scheduler: Scheduler::new(),
            crash_message: None,
            interrupt_requested: false,
            registers: Default::default(),
            object_buffer: Vec::new(),
        }
    }

    pub fn schedule_initial_events(&mut self, now: u64) {
        self.scheduler.schedule(now + 56, LCDControllerEvent::Mode2);
        self.scheduler
            .schedule(now + 56 + 456, LCDControllerEvent::AdvanceLy);
    }

    pub fn schedule_interrupts(&mut self, interrupt_flag: &mut GameBoyFlags<InterruptFlag>) {
        if self.interrupt_requested {
            interrupt_flag.set_flag(InterruptFlag::VerticalBlanking, true);
            self.interrupt_requested = false;
        }
    }

    pub fn deliver_events<R: Renderer>(&mut self, renderer: &mut R, now: u64) {
        while let Some((time, event)) = self.scheduler.poll(now) {
            event.deliver(self, renderer, time);
        }
    }

    fn read_dot_data(
        data: &MemoryChunk,
        character_data_selection: bool,
        character_code: u8,
    ) -> LCDDotData {
        let location = if character_data_selection {
            CHARACTER_DATA_1.start as usize + character_code as usize * 16
        } else {
            CHARACTER_DATA_2.start as usize
                + (((character_code as i8) as isize + 128) as usize) * 16
        };

        LCDDotData {
            data: &data.as_slice()[location..(location + 16)],
        }
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

        self.registers.stat.set_flag(LCDStatusFlag::LYMatch, true);
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
        let x = self.registers.wx.read_value() as i32 - 7;
        let y = self.registers.wy.read_value() as i32;

        return (x, y);
    }

    pub fn crashed(&self) -> bool {
        self.crash_message.is_some()
    }

    fn draw_tiles<R: Renderer>(
        &mut self,
        renderer: &mut R,
        scroll_x: i32,
        scroll_y: i32,
        area_selection: bool,
        character_data_selection: bool,
        wrap: bool,
        transparent: bool,
    ) {
        let ly = self.registers.ly.read_value() as i32;

        if !wrap && ly < scroll_y {
            return;
        }

        let iter = match area_selection {
            false => self.background_display_data_1.as_slice().iter(),
            true => self.background_display_data_2.as_slice().iter(),
        };

        let tile_space_line_height =
            BACKGROUND_DISPLAY_DATA_1.len() as i32 / CHARACTER_AREA_SIZE as i32;
        let mut otile_y = (ly - scroll_y) / CHARACTER_SIZE;

        if scroll_y > ly && (ly - scroll_y) % CHARACTER_SIZE != 0 {
            otile_y -= 1;
        }

        let tile_y = if otile_y < 0 {
            (otile_y % tile_space_line_height) + tile_space_line_height
        } else {
            otile_y % tile_space_line_height
        };
        assert!(tile_y >= 0);

        let iter = iter
            .skip(tile_y as usize * CHARACTER_AREA_SIZE as usize)
            .take(CHARACTER_AREA_SIZE as usize)
            .enumerate();

        for (tile_x, character_code) in iter {
            let character_data = Self::read_dot_data(
                &self.character_data,
                character_data_selection,
                *character_code,
            );
            let x = scroll_x + (tile_x as i32 * CHARACTER_SIZE);
            let y = scroll_y + (otile_y * CHARACTER_SIZE);
            let tile_space_width = CHARACTER_AREA_SIZE as i32 * CHARACTER_SIZE;
            let full_xes = &[x, x - tile_space_width, x + tile_space_width];
            let xes = if wrap {
                full_xes.iter().take(3)
            } else {
                full_xes.iter().take(1)
            };
            for &ix in xes {
                if (ix >= 0 || ix + CHARACTER_SIZE >= 0) && ix < SCREEN_WIDTH {
                    character_data.draw_line(
                        renderer,
                        ix,
                        y,
                        ly,
                        false,
                        false,
                        transparent,
                        &self.registers.bgp,
                    );
                }
            }
        }
    }

    fn draw_oam_data<R: Renderer>(&mut self, renderer: &mut R, priority: ObjectPriority) {
        if !self.registers.lcdc.read_flag(LCDControlFlag::ObjectOn) {
            return;
        }

        let ly = self.registers.ly.read_value() as i32;

        let object_block_composition_selection = self
            .registers
            .lcdc
            .read_flag(LCDControlFlag::ObjectBlockCompositionSelection);

        let sprite_height = if object_block_composition_selection {
            CHARACTER_SIZE * 2
        } else {
            CHARACTER_SIZE
        };

        let iter = LCDObjectIterator::new(&self.oam_data)
            .filter(|o| ly >= o.y && ly < o.y + sprite_height)
            .take(LINE_SPRITE_LIMIT);
        self.object_buffer.clear();
        for obj in iter {
            self.object_buffer.push(obj);
        }
        self.object_buffer
            .sort_by(|a, b| b.x.partial_cmp(&a.x).unwrap());

        for object in &self.object_buffer {
            let low_priority = object.read_flag(LCDObjectAttributeFlag::DisplayPriority);
            if (priority == ObjectPriority::Background) != low_priority {
                continue;
            }

            let vertical_flip = object.read_flag(LCDObjectAttributeFlag::VerticalFlip);
            let horizantal_flip = object.read_flag(LCDObjectAttributeFlag::HorizantalFlip);
            let (y, character_code) =
                object.get_character_data_for_line(ly, object_block_composition_selection);
            let palette = match object.read_flag(LCDObjectAttributeFlag::Palette) {
                false => &self.registers.obp0,
                true => &self.registers.obp1,
            };

            let character_data = Self::read_dot_data(&self.character_data, true, character_code);
            character_data.draw_line(
                renderer,
                object.x,
                y,
                ly,
                vertical_flip,
                horizantal_flip,
                true,
                palette,
            );
        }
    }

    pub fn mode_2(&mut self, time: u64) {
        self.oam_data.borrow();
        self.unusable_memory.borrow();
        self.registers.stat.set_flag_value(LCDStatusFlag::Mode, 0x2);
        self.scheduler
            .schedule(time + 77, LCDControllerEvent::Mode3);
    }

    fn clear_line<R: Renderer>(&mut self, renderer: &mut R) {
        let ly = self.registers.ly.read_value();

        for x in 0..SCREEN_WIDTH {
            renderer.color_pixel(x, ly as i32, color_for_shade::<R>(LCDShade::Shade0));
        }
    }

    fn draw_background<R: Renderer>(&mut self, renderer: &mut R) {
        let bg_area_selection = self
            .registers
            .lcdc
            .read_flag(LCDControlFlag::BGCodeAreaSelection);
        let bg_character_data_selection = self
            .registers
            .lcdc
            .read_flag(LCDControlFlag::BGCharacterDataSelection);
        let (scroll_x, scroll_y) = self.get_scroll_origin_relative_to_lcd();
        self.draw_tiles(
            renderer,
            scroll_x,
            scroll_y,
            bg_area_selection,
            bg_character_data_selection,
            true,
            true,
        );
    }

    fn draw_window<R: Renderer>(&mut self, renderer: &mut R) {
        if !self.registers.lcdc.read_flag(LCDControlFlag::WindowingOn) {
            return;
        }

        let window_area_selection = self
            .registers
            .lcdc
            .read_flag(LCDControlFlag::WindowCodeAreaSelection);
        let (scroll_x, scroll_y) = self.get_window_origin_relative_to_lcd();
        self.draw_tiles(
            renderer,
            scroll_x,
            scroll_y,
            window_area_selection,
            false,
            false,
            false,
        );
    }

    fn mode_3<R: Renderer>(&mut self, renderer: &mut R, time: u64) {
        let ly = self.registers.ly.read_value();
        assert!((ly as i32) < SCREEN_HEIGHT, "drawing ly = {}", ly);
        assert!(self.enabled);

        self.character_data.borrow();
        self.background_display_data_1.borrow();
        self.background_display_data_2.borrow();

        self.clear_line(renderer);
        self.draw_oam_data(renderer, ObjectPriority::Background);
        self.draw_background(renderer);
        self.draw_window(renderer);
        self.draw_oam_data(renderer, ObjectPriority::Foreground);
        self.registers.stat.set_flag_value(LCDStatusFlag::Mode, 0x3);
        self.scheduler
            .schedule(time + 175, LCDControllerEvent::Mode0);
    }

    fn mode_0(&mut self, time: u64) {
        self.character_data.release();
        self.background_display_data_1.release();
        self.background_display_data_2.release();
        self.oam_data.release();
        self.unusable_memory.release();

        self.registers.stat.set_flag_value(LCDStatusFlag::Mode, 0x0);

        if self.registers.ly.read_value() < 143 {
            self.scheduler
                .schedule(time + 204, LCDControllerEvent::Mode2);
        } else {
            self.scheduler
                .schedule(time + 204, LCDControllerEvent::Mode1);
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

        self.scheduler
            .schedule(time + 456, LCDControllerEvent::AdvanceLy);

        if (self.registers.ly.read_value() as i32) < SCREEN_HEIGHT
            && self.registers.ly.read_value() > 0
        {
            self.oam_data.borrow();
            self.unusable_memory.borrow();
        }

        self.registers.stat.set_flag(LCDStatusFlag::LYMatch, false);
        self.scheduler
            .schedule(time + 1, LCDControllerEvent::UpdateLyMatch);
    }

    fn update_ly_match(&mut self, _time: u64) {
        self.registers.stat.set_flag(
            LCDStatusFlag::LYMatch,
            self.registers.ly.read_value() == self.registers.lyc.read_value(),
        );
    }

    fn mode_1<R: Renderer>(&mut self, renderer: &mut R, time: u64) {
        self.registers.stat.set_flag_value(LCDStatusFlag::Mode, 0x1);
        renderer.present();
        self.interrupt_requested = true;

        self.scheduler
            .schedule(time + 4552, LCDControllerEvent::AfterMode1);
    }

    fn after_mode_1(&mut self, time: u64) {
        self.registers.stat.set_flag_value(LCDStatusFlag::Mode, 0x0);
        self.oam_data.borrow();
        self.unusable_memory.borrow();

        self.scheduler.schedule(time + 8, LCDControllerEvent::Mode2);
    }

    fn enable(&mut self, time: u64) {
        assert!(!self.enabled);

        self.enabled = true;
        self.schedule_initial_events(time);
        self.update_ly_match(time);
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

    pub fn save_screenshot<R: Renderer, P: AsRef<std::path::Path>>(
        &self,
        renderer: &mut R,
        path: P,
    ) -> Result<()> {
        renderer.save_buffer(path)?;
        Ok(())
    }

    pub fn tick(&mut self, time: u64) {
        self.check_enabled_state(time);
    }
}
