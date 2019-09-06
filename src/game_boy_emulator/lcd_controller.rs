// Copyright 2018 Remi Bernotavicius

use crate::game_boy_emulator::joypad_register::KeyEvent;
use crate::game_boy_emulator::memory_controller::{
    FlagMask, GameBoyFlags, GameBoyRegister, MemoryChunk, MemoryMappedHardware,
};
use crate::game_boy_emulator::{
    BACKGROUND_DISPLAY_DATA_1, BACKGROUND_DISPLAY_DATA_2, CHARACTER_DATA, CHARACTER_DATA_1,
    CHARACTER_DATA_2, OAM_DATA, UNUSABLE_MEMORY,
};
use crate::rendering::{Color, Event, Renderer};
use crate::util::Scheduler;
use std::iter;

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
    pub bgp: GameBoyFlags<LCDColor>,
    pub obp0: GameBoyFlags<LCDColor>,
    pub obp1: GameBoyFlags<LCDColor>,
    pub wy: GameBoyRegister,
    pub wx: GameBoyRegister,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    y_coordinate: u8,
    x_coordinate: u8,
    character_code: u8,
    flags: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LCDObjectAttributeFlag {
    #[allow(dead_code)]
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
    data: [LCDColor; 64],
}

impl LCDDotData {
    fn new() -> LCDDotData {
        LCDDotData {
            data: [LCDColor::Color0; 64],
        }
    }

    fn draw_line<R: Renderer>(
        &self,
        renderer: &mut R,
        x: i32,
        y: i32,
        ly: u8,
        vertical_flip: bool,
        horizantal_flip: bool,
        enable_transparency: bool,
        palette: &GameBoyFlags<LCDColor>,
    ) {
        assert!(ly as i32 >= y && (ly as i32) < y + CHARACTER_SIZE as i32);

        let target_line = if vertical_flip {
            y + CHARACTER_SIZE as i32 - 1 - ly as i32
        } else {
            ly as i32 - y
        };
        let start_pixel = (target_line * CHARACTER_SIZE as i32) as usize;
        let end_pixel = start_pixel + CHARACTER_SIZE as usize;
        let iter = self.data[start_pixel..end_pixel].iter().enumerate();
        for (mut offset_x, &color) in iter {
            if horizantal_flip {
                offset_x = CHARACTER_SIZE as usize - offset_x;
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

pub struct LCDController<'a, R> {
    renderer: R,
    pub crash_message: Option<String>,
    pub character_data: MemoryChunk,
    pub background_display_data_1: MemoryChunk,
    pub background_display_data_2: MemoryChunk,
    pub oam_data: MemoryChunk,
    pub unusable_memory: MemoryChunk,
    pub registers: LCDControllerRegisters,
    scheduler: Scheduler<LCDController<'a, R>>,
    enabled: bool,
    interrupt_requested: bool,
}

impl<'a, R: Renderer> LCDController<'a, R> {
    pub fn new(renderer: R) -> Self {
        LCDController {
            character_data: MemoryChunk::from_range(CHARACTER_DATA),
            background_display_data_1: MemoryChunk::from_range(BACKGROUND_DISPLAY_DATA_1),
            background_display_data_2: MemoryChunk::from_range(BACKGROUND_DISPLAY_DATA_2),
            oam_data: MemoryChunk::from_range(OAM_DATA),
            unusable_memory: MemoryChunk::from_range(UNUSABLE_MEMORY),
            enabled: true,
            renderer,
            scheduler: Scheduler::new(),
            crash_message: None,
            interrupt_requested: false,
            registers: Default::default(),
        }
    }

    pub fn schedule_initial_events(&mut self, now: u64) {
        self.scheduler.schedule(now + 56 + 4, Self::mode_2);
        self.scheduler.schedule(now + 56 + 456, Self::advance_ly);
    }

    pub fn schedule_interrupts(&mut self, interrupt_flag: &mut GameBoyFlags<InterruptFlag>) {
        if self.interrupt_requested {
            interrupt_flag.set_flag(InterruptFlag::VerticalBlanking, true);
            self.interrupt_requested = false;
        }
    }

    pub fn deliver_events(&mut self, now: u64) {
        while let Some((time, event)) = self.scheduler.poll(now) {
            event(self, time);
        }
    }

    fn read_dot_data(&self, character_data_selection: bool, character_code: u8) -> LCDDotData {
        let mut dot_data = LCDDotData::new();

        let location = if character_data_selection {
            CHARACTER_DATA_1.start as usize + character_code as usize * 16
        } else {
            CHARACTER_DATA_2.start as usize
                + (((character_code as i8) as isize + 128) as usize) * 16
        };

        let mut iter = self.character_data.as_slice()[location..]
            .iter()
            .take(16)
            .peekable();

        let mut i = 0;
        while iter.peek() != None {
            let byte1: u8 = *iter.next().unwrap();
            let byte2: u8 = *iter.next().unwrap();
            for bit in (0..8).rev() {
                let shade_upper = ((byte2 >> bit) & 0x1) << 1;
                let shade_lower = (byte1 >> bit) & 0x1;
                dot_data.data[i] = match shade_upper | shade_lower {
                    0x0 => LCDColor::Color0,
                    0x1 => LCDColor::Color1,
                    0x2 => LCDColor::Color2,
                    0x3 => LCDColor::Color3,
                    _ => panic!(),
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

    pub fn poll_renderer(&mut self) -> Vec<KeyEvent> {
        let mut key_events = vec![];

        for event in self.renderer.poll_events() {
            match event {
                Event::Quit { .. } => self.crash_message = Some(String::from("Screen Closed")),
                Event::KeyDown(code) => key_events.push(KeyEvent::Down(code)),
                Event::KeyUp(code) => key_events.push(KeyEvent::Up(code)),
            }
        }

        key_events
    }

    fn draw_tiles(
        &mut self,
        scroll_x: i32,
        scroll_y: i32,
        area_selection: bool,
        character_data_selection: bool,
        wrap: bool,
    ) {
        let ly = self.registers.ly.read_value();

        if !wrap && (ly as i32) < scroll_y {
            return;
        }

        let iter = match area_selection {
            false => self.background_display_data_1.as_slice().iter(),
            true => self.background_display_data_2.as_slice().iter(),
        };

        let tile_space_line_height =
            BACKGROUND_DISPLAY_DATA_1.len() as i32 / CHARACTER_AREA_SIZE as i32;
        let mut otile_y = (ly as i32 - scroll_y) / CHARACTER_SIZE as i32;

        if scroll_y > ly as i32 && (ly as i32 - scroll_y) % CHARACTER_SIZE as i32 != 0 {
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
            let character_data = self.read_dot_data(character_data_selection, *character_code);
            let x = scroll_x + (tile_x as i32 * CHARACTER_SIZE as i32);
            let y = scroll_y + (otile_y * CHARACTER_SIZE as i32);
            let tile_space_width = CHARACTER_AREA_SIZE as i32 * CHARACTER_SIZE as i32;
            let full_xes = &[x, x - tile_space_width, x + tile_space_width];
            let xes = if wrap {
                full_xes.iter().take(3)
            } else {
                full_xes.iter().take(1)
            };
            for &ix in xes {
                if (ix >= 0 || ix + CHARACTER_SIZE as i32 >= 0) && ix < 160 {
                    character_data.draw_line(
                        &mut self.renderer,
                        ix,
                        y,
                        ly,
                        false,
                        false,
                        false,
                        &self.registers.bgp,
                    );
                }
            }
        }
    }

    fn draw_oam_data(&mut self) {
        if !self.registers.lcdc.read_flag(LCDControlFlag::ObjectOn) {
            return;
        }

        let ly = self.registers.ly.read_value();

        let object_block_composition_selection = self
            .registers
            .lcdc
            .read_flag(LCDControlFlag::ObjectBlockCompositionSelection);

        let sprite_height = if object_block_composition_selection {
            CHARACTER_SIZE as i32 * 2
        } else {
            CHARACTER_SIZE as i32
        };

        let iter = LCDObjectIterator::new(&self.oam_data);
        for object in iter {
            let x = object.x_coordinate as i32 - CHARACTER_SIZE as i32;
            let y = object.y_coordinate as i32 - (CHARACTER_SIZE as i32 * 2);
            if ly as i32 >= y && (ly as i32) < y + sprite_height {
                let vertical_flip = object.read_flag(LCDObjectAttributeFlag::VerticalFlip);
                let horizantal_flip = object.read_flag(LCDObjectAttributeFlag::HorizantalFlip);

                let (y, character_code) = if object_block_composition_selection {
                    let first_code = object.character_code & !1;
                    let second_code = object.character_code | 1;

                    if (ly as i32) < y + CHARACTER_SIZE as i32 {
                        (
                            y,
                            if vertical_flip {
                                second_code
                            } else {
                                first_code
                            },
                        )
                    } else {
                        (
                            y + CHARACTER_SIZE as i32,
                            if vertical_flip {
                                first_code
                            } else {
                                second_code
                            },
                        )
                    }
                } else {
                    (y, object.character_code)
                };

                let palette = match object.read_flag(LCDObjectAttributeFlag::Palette) {
                    false => &self.registers.obp0,
                    true => &self.registers.obp1,
                };

                let character_data = self.read_dot_data(true, character_code);
                character_data.draw_line(
                    &mut self.renderer,
                    x,
                    y,
                    ly,
                    vertical_flip,
                    horizantal_flip,
                    true,
                    palette,
                );
            }
        }
    }

    fn update_screen(&mut self) {
        self.renderer.present()
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

    fn draw_background(&mut self) {
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
            scroll_x,
            scroll_y,
            bg_area_selection,
            bg_character_data_selection,
            true,
        );
    }

    fn draw_window(&mut self) {
        if !self.registers.lcdc.read_flag(LCDControlFlag::WindowingOn) {
            return;
        }

        let window_area_selection = self
            .registers
            .lcdc
            .read_flag(LCDControlFlag::WindowCodeAreaSelection);
        let (scroll_x, scroll_y) = self.get_window_origin_relative_to_lcd();
        self.draw_tiles(scroll_x, scroll_y, window_area_selection, false, false);
    }

    fn mode_3(&mut self, time: u64) {
        self.character_data.borrow();
        self.background_display_data_1.borrow();
        self.background_display_data_2.borrow();

        self.draw_background();
        self.draw_window();
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
        } else if self.registers.ly.read_value() == 0 {
            self.scheduler.schedule(time + 904, Self::advance_ly);
        } else {
            self.scheduler.schedule(time + 456, Self::advance_ly);
        }

        if self.registers.ly.read_value() < 144 && self.registers.ly.read_value() > 0 {
            self.oam_data.borrow();
            self.unusable_memory.borrow();
        }

        self.registers.stat.set_flag(LCDStatusFlag::LYMatch, false);
        self.scheduler.schedule(time + 1, Self::update_ly_match);
    }

    fn update_ly_match(&mut self, _time: u64) {
        self.registers.stat.set_flag(
            LCDStatusFlag::LYMatch,
            self.registers.ly.read_value() == self.registers.lyc.read_value(),
        );
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

    pub fn tick(&mut self, time: u64) {
        self.check_enabled_state(time);
    }
}
