// Copyright 2018 Remi Bernotavicius

//! This module contains an emulator for the LCD and PPU of the Game Boy.
//!
//! The LCD Controller draws a picture on the screen. It gets its data about what to draw from
//! several memory-mapped chunks of data. The program writes to those sections of memory (but only
//! when the LCD Controller isn't using it).
//!
//! The LCD Controller runs in a loop drawing the screen one horizontal line at a time. When it is
//! actively drawing a line, its sections of memory are not available to the program. The program
//! can only access the data during designated times. These times are known as the horizontal
//! blanking period and the vertical blanking period.
//!
//! The LCD Controller denotes the various periods of time interesting to the program as "modes".
//!
//! - Mode 0: Horizontal blanking period
//! - Mode 1: Vertical blanking period
//! - Mode 2: Reading OAM period
//! - Mode 3: Drawing
//!
//! It loops through Mode 2, Mode 3, Mode 0 for each horizontal line, and then it goes to Mode 1
//! before going back to Mode 2 repeating the cycle. Most programs will want to do their work
//! updating the data the LCD Controller reads during Mode 1, which is the longest mode.
//!
//! Here is a list of various terms used throughout.
//!
//! - LCD: Liquid Crystal Display; This is the type of display in the Game Boy.
//! - Background: This is layer displayed on the screen consisting of an 2D array of tiles. It can
//!               be scrolled using SCX and SCY registers.
//! - Objects: These are sprites. It consists of 1 or 2 tiles (character data) and can be
//!            independently placed on the screen.
//! - Window: This is another layer displayed on the screen usually in front the background and
//!           sprites. It can be scrolled independently of the background.
//! - Dot Data: This is just an ordered (by position) array of dots (pixels). Basically a kind of
//!             bitmap. It describes the color (palette entry) of each pixel.
//! - Character Data: This is an 8x8 tile used for drawing sprites. It is represented as dot data.
//! - OAM: Object Attribute Memory; This chunk of memory contains information about objects
//!        (sprites) like position etc.
//! - Background Display Data: This chunk of memory contains tiles for the background and window.
//!                            The tiles are represented as dot data.
//! - Horizontal Blanking Period: A period of time between when two lines of the screen are being
//!                               drawn.
//! - Vertical Blanking Period: A period of time between when the last line of the screen is drawn
//!                             and the first line of the screen is drawn.
//!

use crate::game_boy_emulator::memory_controller::{
    FlagMask, GameBoyFlags, GameBoyRegister, MemoryChunk, MemoryMappedHardware,
};
use crate::game_boy_emulator::{observe, InterruptFlag, PerfObserver};
use crate::rendering::{Color, Renderer};
use crate::util::Scheduler;
use alloc::vec::Vec;
use core::ops::Range;
use core::{fmt, iter};
use enum_iterator::IntoEnumIterator;
use num_enum::IntoPrimitive;
use serde_derive::{Deserialize, Serialize};
use strum_macros::IntoStaticStr;

/// The width of the screen in pixels
const SCREEN_WIDTH: i32 = 160;

/// The height of the screen in pixels
const SCREEN_HEIGHT: i32 = 144;

/// This is the size (width and height) in pixels of one piece of character data.
const CHARACTER_SIZE: i32 = 8;

/// Character data in memory (called a character area) is sometimes indexed by coordinates. This is
/// the width of such a space (in number of character data)
const CHARACTER_AREA_SIZE: u16 = 32;

/// This is the address range in memory where the character data is stored.
const CHARACTER_DATA: Range<u16> = Range {
    start: 0x8000,
    end: 0x9800,
};

/// This range is the first section of character data. These are offsets from the start of
/// character data section of memory.
const CHARACTER_DATA_1: Range<u16> = Range {
    start: 0x0,
    end: 0x1000,
};

/// This range is the second section of character data. These are offsets from the start of
/// character data section of memory.
const CHARACTER_DATA_2: Range<u16> = Range {
    start: 0x800,
    end: 0x1800,
};

/// This is the address range of memory where the first section of background display data is stored.
const BACKGROUND_DISPLAY_DATA_1: Range<u16> = Range {
    start: 0x9800,
    end: 0x9C00,
};

/// This is the address range of memory where the second section of background display data is stored.
const BACKGROUND_DISPLAY_DATA_2: Range<u16> = Range {
    start: 0x9C00,
    end: 0xA000,
};

/// This is the address range of memory where object attribute data is stored.
pub const OAM_DATA: Range<u16> = Range {
    start: 0xFE00,
    end: 0xFEA0,
};

/// This is an address range of memory where an unusable chunk of memory is stored.
const UNUSABLE_MEMORY: Range<u16> = Range {
    start: 0xFEA0,
    end: 0xFF00,
};

/// The maximum number of sprites that can appear on one line
const LINE_SPRITE_LIMIT: usize = 10;

/// This register is used to control DMA (direct memory access). It allows bulk transfers of
/// memory.
#[derive(Default, Serialize, Deserialize)]
pub struct DmaRegister {
    value: u8,
    requested: bool,
}

impl MemoryMappedHardware for DmaRegister {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_value(&self, _: u16) -> u8 {
        self.read_value()
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_value(&mut self, _: u16, value: u8) {
        self.set_value(value);
        self.requested = true;
    }
}

impl fmt::Debug for DmaRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:02x}", self.value)
    }
}

impl DmaRegister {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn take_request(&mut self) -> Option<u16> {
        if self.requested {
            self.requested = false;
            Some(self.read_value() as u16 * 0x100)
        } else {
            None
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn read_value(&self) -> u8 {
        self.value
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LcdControllerRegisters {
    /// The LCDC (LCD control) register.
    pub lcdc: GameBoyFlags<LcdControlFlag>,

    /// The LCD status register.
    pub stat: GameBoyFlags<LcdStatusFlag>,

    /// SCY (scroll Y) register. Specifies the position of the visible portion of the background.
    pub scy: GameBoyRegister,

    /// SCX (scroll X) register. Specifies the position of the visible portion of the background.
    pub scx: GameBoyRegister,

    /// The LY register. This is the horizontal line currently being drawn.
    pub ly: GameBoyRegister,

    /// LY compare register. Can be used by programs to be notified or detect when a certain
    /// horizontal line is being drawn.
    pub lyc: GameBoyRegister,

    /// DMA (direct memory access) transfer register. Can be used by programs to do bulk transfers
    /// of memory.
    pub dma: DmaRegister,

    /// BGP (background palette) register. The palette for background tiles.
    pub bgp: GameBoyFlags<LcdColor>,

    /// OBP0 (object palette) register. First palette for objects (sprites).
    pub obp0: GameBoyFlags<LcdColor>,

    /// OBP1 (object palette) register. Second palette for objects (sprites).
    pub obp1: GameBoyFlags<LcdColor>,

    /// WY (window Y) register. The Y position of the window.
    pub wy: GameBoyRegister,

    /// WX (window X) reigster. The X position of the window.
    pub wx: GameBoyRegister,
}

/// Tiles and objects (sprites) pixels are described using these values. The actual color they
/// represent depends on the palette.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, Serialize, Deserialize, IntoEnumIterator,
)]
#[repr(u8)]
pub enum LcdColor {
    Color3 = 0b11000000,
    Color2 = 0b00110000,
    Color1 = 0b00001100,
    Color0 = 0b00000011,
}

impl FlagMask for LcdColor {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_mask() -> u8 {
        0xFF
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn write_mask() -> u8 {
        0xFF
    }
}

/// These are the 4 shades that the Game Boy (DMG) screen is capable of displaying.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LcdShade {
    Shade0 = 0x0,
    Shade1 = 0x1,
    Shade2 = 0x2,
    Shade3 = 0x3,
}

#[cfg_attr(not(debug_assertions), inline(always))]
fn color_for_shade<R: Renderer>(shade: LcdShade) -> R::Color {
    match shade {
        LcdShade::Shade0 => R::Color::new(0xe0, 0xf8, 0xd0),
        LcdShade::Shade1 => R::Color::new(0x88, 0xc0, 0x70),
        LcdShade::Shade2 => R::Color::new(0x34, 0x68, 0x56),
        LcdShade::Shade3 => R::Color::new(0x08, 0x18, 0x20),
    }
}

/// This is a mask for the LCDC (LCD control) register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, IntoEnumIterator)]
#[repr(u8)]
pub enum LcdControlFlag {
    /// Controls whether the LCD is on and the PPU is running.
    DisplayOn = 0b10000000,

    /// Controls which background display data the window is using.
    /// (0 = BACKGROUND_DISPLAY_DATA_1, 1 = BACKGROUND_DISPLAY_DATA_2)
    WindowCodeAreaSelection = 0b01000000,

    /// Controls whether the window is displayed or not.
    WindowingOn = 0b00100000,

    /// Controls what character data the background and window use.
    /// (0 = CHARACTER_DATA_2, 1 = CHARACTER_DATA_1)
    BGCharacterDataSelection = 0b00010000,

    /// Controls which background display data the background is using.
    /// (0 = BACKGROUND_DISPLAY_DATA_1, 1 = BACKGROUND_DISPLAY_DATA_2)
    BGCodeAreaSelection = 0b00001000,

    /// This controls the size of objects (sprites).
    /// (0 = 2 * CHARACTER_SIZE (16), 1 = CHARACTER_SIZE (8))
    ObjectBlockCompositionSelection = 0b00000100,

    /// This controls whether the objects (sprites) are visible. (0 = not visible, 1 = visible)
    ObjectOn = 0b00000010,

    /// This controls whether the background is visible. (0 = not visible, 1 = visible)
    BGDisplayOn = 0b00000001,
}

impl FlagMask for LcdControlFlag {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_mask() -> u8 {
        0xFF
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn write_mask() -> u8 {
        0xFF
    }
}

/// This is a mask for the STAT register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, IntoEnumIterator)]
#[repr(u8)]
pub enum LcdStatusFlag {
    /// Enable interrupt when LCY == LY. (0 = disable, 1 = enable)
    InterruptLYMatching = 0b01000000,

    /// Enable interrupt when mode 2 happens. (0 = disable, 1 = enable)
    InterruptMode10 = 0b00100000,

    /// Enable interrupt when mode 1 happens. (0 = disable, 1 = enable)
    InterruptMode01 = 0b00010000,

    /// Enable interrupt when mode 0 happens. (0 = disable, 1 = enable)
    InterruptMode00 = 0b00001000,

    /// 1 when LYC == LY. 0 otherwise
    LYMatch = 0b00000100,

    /// The current mode (operation state) of the LCD.
    Mode = 0b00000011,
}

impl FlagMask for LcdStatusFlag {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_mask() -> u8 {
        Self::InterruptLYMatching as u8
            | Self::InterruptMode10 as u8
            | Self::InterruptMode01 as u8
            | Self::InterruptMode00 as u8
            | Self::LYMatch as u8
            | Self::Mode as u8
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn write_mask() -> u8 {
        Self::InterruptLYMatching as u8
            | Self::InterruptMode10 as u8
            | Self::InterruptMode01 as u8
            | Self::InterruptMode00 as u8
    }
}

/// This represents an object (sprite).
struct LcdObject {
    y: i32,
    x: i32,
    character_code: u8,
    flags: GameBoyFlags<LcdObjectAttributeFlag>,
}

/// Mask for LcdObject flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u8)]
enum LcdObjectAttributeFlag {
    /// Controls whether the object is displayed in front or behind the background and window.
    /// (0 = in front, 1 = behind)
    DisplayPriority = 0b10000000,

    /// Flips the object vertically. (0 = no flip, 1 = flip)
    VerticalFlip = 0b01000000,

    /// Flips the object horizontally. (0 = no flip, 1 = flip)
    HorizontalFlip = 0b00100000,

    /// The palette to be used.
    Palette = 0b00010000,
}

impl FlagMask for LcdObjectAttributeFlag {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_mask() -> u8 {
        0xFF
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn write_mask() -> u8 {
        0xFF
    }
}

impl LcdObject {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_flag(&self, flag: LcdObjectAttributeFlag) -> bool {
        self.flags.read_flag(flag)
    }

    /// Returns tuple (y position, character code)
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn get_character_data_for_line(
        &self,
        line: i32,
        object_block_composition_selection: bool,
    ) -> (i32, u8) {
        // If we are a double-height sprite
        if object_block_composition_selection {
            let first_code = self.character_code & !1;
            let second_code = self.character_code | 1;
            let vertical_flip = self.read_flag(LcdObjectAttributeFlag::VerticalFlip);

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

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn draw_line(
        &self,
        line: &mut ScanLine,
        character_data: &MemoryChunk,
        palette: &GameBoyFlags<LcdColor>,
        object_block_composition_selection: bool,
        ly: i32,
    ) {
        let behind_bg = self.read_flag(LcdObjectAttributeFlag::DisplayPriority);
        let vertical_flip = self.read_flag(LcdObjectAttributeFlag::VerticalFlip);
        let horizantal_flip = self.read_flag(LcdObjectAttributeFlag::HorizontalFlip);
        let (y, character_code) =
            self.get_character_data_for_line(ly, object_block_composition_selection);

        let character_data = LcdController::read_dot_data(character_data, true, character_code);
        character_data.draw_line(
            line,
            self.x,
            y,
            ly,
            behind_bg,
            vertical_flip,
            horizantal_flip,
            true,
            palette,
        );
    }
}

struct LcdObjectIterator<'a> {
    chunk_iterator: iter::Peekable<core::slice::Iter<'a, u8>>,
}

impl<'a> Iterator for LcdObjectIterator<'a> {
    type Item = LcdObject;

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn next(&mut self) -> Option<LcdObject> {
        if self.chunk_iterator.peek().is_none() {
            None
        } else {
            let y = *self.chunk_iterator.next().unwrap() as i32 - CHARACTER_SIZE * 2;
            let x = *self.chunk_iterator.next().unwrap() as i32 - CHARACTER_SIZE;
            let character_code = *self.chunk_iterator.next().unwrap();
            let mut flags = GameBoyFlags::new();
            flags.set_value(*self.chunk_iterator.next().unwrap());

            Some(LcdObject {
                x,
                y,
                character_code,
                flags,
            })
        }
    }
}

impl<'a> LcdObjectIterator<'a> {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn new(chunk: &'a MemoryChunk) -> LcdObjectIterator<'a> {
        LcdObjectIterator {
            chunk_iterator: chunk.as_slice().iter().peekable(),
        }
    }
}

/// Dot data is basically just an array of pixels. Character data is comprised of it.
struct LcdDotData<'a> {
    data: &'a [u8],
}

impl<'a> LcdDotData<'a> {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_pixel(&self, offset: usize) -> LcdColor {
        let byte_offset = (offset / 8) * 2;
        let bit_offset = 7 - (offset % 8);

        let byte1 = self.data[byte_offset];
        let byte2 = self.data[byte_offset + 1];

        let shade_upper = ((byte2 >> bit_offset) & 0x1) << 1;
        let shade_lower = (byte1 >> bit_offset) & 0x1;
        match shade_upper | shade_lower {
            0x0 => LcdColor::Color0,
            0x1 => LcdColor::Color1,
            0x2 => LcdColor::Color2,
            0x3 => LcdColor::Color3,
            _ => panic!(),
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[allow(clippy::too_many_arguments)]
    fn draw_line(
        &self,
        line: &mut ScanLine,
        x: i32,
        y: i32,
        ly: i32,
        behind_bg: bool,
        vertical_flip: bool,
        horizantal_flip: bool,
        enable_transparency: bool,
        palette: &GameBoyFlags<LcdColor>,
    ) {
        assert!(ly >= y && ly < y + CHARACTER_SIZE);
        assert!(ly < SCREEN_HEIGHT, "{}", "drawing ly = {ly}");

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
            let x = x + offset_x as i32;
            if x < 0 {
                continue;
            } else if x >= SCREEN_WIDTH {
                break;
            }
            if color != LcdColor::Color0 || !enable_transparency {
                let shade = match palette.read_flag_value(color) {
                    0x0 => LcdShade::Shade0,
                    0x1 => LcdShade::Shade1,
                    0x2 => LcdShade::Shade2,
                    0x3 => LcdShade::Shade3,
                    _ => panic!(),
                };
                if !behind_bg || line.get_pixel(x) == LcdShade::Shade0 {
                    line.set_pixel(x, shade);
                }
            }
        }
    }
}

/// Used with the scheduler to run functions after some amount of time.
#[derive(Serialize, Deserialize, IntoStaticStr)]
enum LcdControllerEvent {
    AdvanceLy,
    Mode0,
    Mode1,
    Mode2,
    Mode3,
    UpdateLyMatch,
}

impl LcdControllerEvent {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn deliver<R: Renderer>(self, controller: &mut LcdController, renderer: &mut R, time: u64) {
        match self {
            Self::AdvanceLy => controller.advance_ly(time),
            Self::Mode0 => controller.mode_0(time),
            Self::Mode1 => controller.mode_1(renderer, time),
            Self::Mode2 => controller.mode_2(time),
            Self::Mode3 => controller.mode_3(renderer, time),
            Self::UpdateLyMatch => controller.update_ly_match(time),
        }
    }
}

struct ScanLine {
    data: [LcdShade; SCREEN_WIDTH as usize],
}

impl ScanLine {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn new() -> Self {
        Self {
            data: [LcdShade::Shade0; SCREEN_WIDTH as usize],
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn set_pixel(&mut self, x: i32, shade: LcdShade) {
        assert!(x >= 0 && x < self.data.len() as i32);
        self.data[x as usize] = shade
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn get_pixel(&self, x: i32) -> LcdShade {
        assert!(x >= 0 && x < self.data.len() as i32);
        self.data[x as usize]
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn draw<R: Renderer>(&self, renderer: &mut R, y: i32) {
        for (x, &v) in self.data.iter().enumerate() {
            renderer.color_pixel(x as i32, y, color_for_shade::<R>(v));
        }
    }
}

/// An emulator for the LCD and PPU of the Game Boy.
#[derive(Serialize, Deserialize)]
pub struct LcdController {
    pub character_data: MemoryChunk,
    pub background_display_data_1: MemoryChunk,
    pub background_display_data_2: MemoryChunk,
    pub oam_data: MemoryChunk,
    pub unusable_memory: MemoryChunk,
    pub registers: LcdControllerRegisters,
    scheduler: Scheduler<LcdControllerEvent>,
    enabled: bool,
    vertical_blanking_interrupt: bool,
    stat_interrupt: bool,
    #[serde(skip)]
    object_buffer: Vec<LcdObject>,
}

impl LcdController {
    pub fn new() -> Self {
        LcdController {
            character_data: MemoryChunk::from_range(CHARACTER_DATA),
            background_display_data_1: MemoryChunk::from_range(BACKGROUND_DISPLAY_DATA_1),
            background_display_data_2: MemoryChunk::from_range(BACKGROUND_DISPLAY_DATA_2),
            oam_data: MemoryChunk::from_range(OAM_DATA),
            unusable_memory: MemoryChunk::from_range(UNUSABLE_MEMORY),
            enabled: true,
            scheduler: Scheduler::new(),
            vertical_blanking_interrupt: false,
            stat_interrupt: false,
            registers: Default::default(),
            object_buffer: Vec::new(),
        }
    }

    /// Must be called after creation to schedule events needed for proper operation.
    pub fn schedule_initial_events(&mut self, now: u64) {
        self.scheduler.schedule(now + 56, LcdControllerEvent::Mode2);
        self.scheduler
            .schedule(now + 56 + 456, LcdControllerEvent::AdvanceLy);
    }

    /// Must be called periodically so the proper interrupts can be triggered.
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn schedule_interrupts(&mut self, interrupt_flag: &mut GameBoyFlags<InterruptFlag>) {
        if self.vertical_blanking_interrupt {
            interrupt_flag.set_flag(InterruptFlag::VerticalBlanking, true);
            self.vertical_blanking_interrupt = false;
        }
        if self.stat_interrupt {
            interrupt_flag.set_flag(InterruptFlag::LCDSTAT, true);
            self.stat_interrupt = false;
        }
    }

    /// Should be called periodically to drive the emulator.
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn deliver_events(
        &mut self,
        renderer: &mut impl Renderer,
        observer: &mut impl PerfObserver,
        now: u64,
    ) {
        while let Some((time, event)) = self.scheduler.poll(now) {
            observe(observer, (&event).into(), || {
                event.deliver(self, renderer, time)
            });
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn read_dot_data(
        data: &MemoryChunk,
        character_data_selection: bool,
        character_code: u8,
    ) -> LcdDotData {
        let location = if character_data_selection {
            CHARACTER_DATA_1.start as usize + character_code as usize * 16
        } else {
            CHARACTER_DATA_2.start as usize
                + (((character_code as i8) as isize + 128) as usize) * 16
        };

        LcdDotData {
            data: &data.as_slice()[location..(location + 16)],
        }
    }

    /// Resets the state back to what it is when the Game Boy boots
    pub fn set_state_post_bios(&mut self) {
        self.registers
            .lcdc
            .set_flag(LcdControlFlag::DisplayOn, true);
        self.registers
            .lcdc
            .set_flag(LcdControlFlag::BGCharacterDataSelection, true);
        self.registers
            .lcdc
            .set_flag(LcdControlFlag::BGDisplayOn, true);
        self.registers.bgp.set_value(0xFC);

        self.registers.stat.set_flag(LcdStatusFlag::LYMatch, true);
        self.registers.stat.set_flag_value(LcdStatusFlag::Mode, 0x1);
        self.registers.dma.set_value(0xff);
        self.registers.obp0.set_value(0xff);
        self.registers.obp1.set_value(0xff);

        let character_ram = include_bytes!("../assets/character_ram.bin");
        self.character_data
            .clone_range_from_slice(0..character_ram.len(), &character_ram[..]);

        let background_display_data = include_bytes!("../assets/background_display_data.bin");

        self.background_display_data_1.clone_range_from_slice(
            0x100..0x100 + background_display_data.len(),
            &background_display_data[..],
        );

        let oam_data = include_bytes!("../assets/oam_data.bin");

        self.oam_data.clone_from_slice(&oam_data[..]);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn get_scroll_origin_relative_to_lcd(&self) -> (i32, i32) {
        let mut x = -(self.registers.scx.read_value() as i32);
        let mut y = -(self.registers.scy.read_value() as i32);

        /*
         * This supports the behavior of the background wrapping
         */
        if x < -128 {
            x += 256;
        }

        if y < -128 {
            y += 256;
        }

        (x, y)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn get_window_origin_relative_to_lcd(&self) -> (i32, i32) {
        let x = self.registers.wx.read_value() as i32 - 7;
        let y = self.registers.wy.read_value() as i32;

        (x, y)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn draw_tiles(
        &mut self,
        line: &mut ScanLine,
        scroll_x: i32,
        scroll_y: i32,
        area_selection: bool,
        character_data_selection: bool,
        wrap: bool,
    ) {
        let ly = self.registers.ly.read_value() as i32;

        if !wrap && ly < scroll_y {
            return;
        }

        let bg_data_slice = match area_selection {
            false => self.background_display_data_1.as_slice(),
            true => self.background_display_data_2.as_slice(),
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

        let start_index = tile_y as usize * CHARACTER_AREA_SIZE as usize;
        let end_index = start_index + CHARACTER_AREA_SIZE as usize;
        let iter = bg_data_slice[start_index..end_index].iter().enumerate();

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
                        line,
                        ix,
                        y,
                        ly,
                        false,
                        false,
                        false,
                        false,
                        &self.registers.bgp,
                    );
                }
            }
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn draw_oam_data(&mut self, line: &mut ScanLine) {
        if !self.registers.lcdc.read_flag(LcdControlFlag::ObjectOn) {
            return;
        }

        let ly = self.registers.ly.read_value() as i32;

        let object_block_composition_selection = self
            .registers
            .lcdc
            .read_flag(LcdControlFlag::ObjectBlockCompositionSelection);

        let sprite_height = if object_block_composition_selection {
            CHARACTER_SIZE * 2
        } else {
            CHARACTER_SIZE
        };

        let iter = LcdObjectIterator::new(&self.oam_data)
            .filter(|o| ly >= o.y && ly < o.y + sprite_height)
            .take(LINE_SPRITE_LIMIT);
        self.object_buffer.clear();
        for obj in iter {
            self.object_buffer.push(obj);
        }
        self.object_buffer
            .sort_by(|a, b| b.x.partial_cmp(&a.x).unwrap());

        for object in &self.object_buffer {
            let palette = match object.read_flag(LcdObjectAttributeFlag::Palette) {
                false => &self.registers.obp0,
                true => &self.registers.obp1,
            };
            object.draw_line(
                line,
                &self.character_data,
                palette,
                object_block_composition_selection,
                ly,
            );
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn mode_2(&mut self, time: u64) {
        self.oam_data.borrow();
        self.unusable_memory.borrow();
        self.registers.stat.set_flag_value(LcdStatusFlag::Mode, 0x2);
        if self
            .registers
            .stat
            .read_flag(LcdStatusFlag::InterruptMode10)
        {
            self.stat_interrupt = true;
        }

        self.scheduler
            .schedule(time + 77, LcdControllerEvent::Mode3);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn draw_background(&mut self, line: &mut ScanLine) {
        let bg_area_selection = self
            .registers
            .lcdc
            .read_flag(LcdControlFlag::BGCodeAreaSelection);
        let bg_character_data_selection = self
            .registers
            .lcdc
            .read_flag(LcdControlFlag::BGCharacterDataSelection);
        let (scroll_x, scroll_y) = self.get_scroll_origin_relative_to_lcd();
        self.draw_tiles(
            line,
            scroll_x,
            scroll_y,
            bg_area_selection,
            bg_character_data_selection,
            true,
        );
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn draw_window(&mut self, line: &mut ScanLine) {
        if !self.registers.lcdc.read_flag(LcdControlFlag::WindowingOn) {
            return;
        }

        let window_area_selection = self
            .registers
            .lcdc
            .read_flag(LcdControlFlag::WindowCodeAreaSelection);
        let (scroll_x, scroll_y) = self.get_window_origin_relative_to_lcd();
        self.draw_tiles(
            line,
            scroll_x,
            scroll_y,
            window_area_selection,
            false,
            false,
        );
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn mode_3<R: Renderer>(&mut self, renderer: &mut R, time: u64) {
        let ly = self.registers.ly.read_value();
        assert!((ly as i32) < SCREEN_HEIGHT, "{}", "drawing ly = {ly}");
        assert!(self.enabled);

        self.character_data.borrow();
        self.background_display_data_1.borrow();
        self.background_display_data_2.borrow();

        let mut line = ScanLine::new();
        self.draw_background(&mut line);
        self.draw_window(&mut line);
        self.draw_oam_data(&mut line);
        line.draw(renderer, ly as i32);

        self.registers.stat.set_flag_value(LcdStatusFlag::Mode, 0x3);
        self.scheduler
            .schedule(time + 175, LcdControllerEvent::Mode0);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn mode_0(&mut self, time: u64) {
        self.character_data.release();
        self.background_display_data_1.release();
        self.background_display_data_2.release();
        self.oam_data.release();
        self.unusable_memory.release();

        self.registers.stat.set_flag_value(LcdStatusFlag::Mode, 0x0);

        if self
            .registers
            .stat
            .read_flag(LcdStatusFlag::InterruptMode00)
        {
            self.stat_interrupt = true;
        }

        if self.registers.ly.read_value() < 143 {
            self.scheduler
                .schedule(time + 204, LcdControllerEvent::Mode2);
        } else {
            self.scheduler
                .schedule(time + 204, LcdControllerEvent::Mode1);
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn advance_ly(&mut self, time: u64) {
        // This advances the ly register, which represents the horizontal line that is currently
        // being drawn on the LCD.
        self.registers.ly.add(1);

        // There are only 154 lines, so wrap back to zero after that.
        if self.registers.ly.read_value() > 153 {
            self.registers.ly.set_value(0);
        }

        self.scheduler
            .schedule(time + 456, LcdControllerEvent::AdvanceLy);

        self.registers.stat.set_flag(LcdStatusFlag::LYMatch, false);
        self.scheduler
            .schedule(time + 1, LcdControllerEvent::UpdateLyMatch);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn update_ly_match(&mut self, _time: u64) {
        if self.registers.ly.read_value() == self.registers.lyc.read_value() {
            self.registers.stat.set_flag(LcdStatusFlag::LYMatch, true);
            if self
                .registers
                .stat
                .read_flag(LcdStatusFlag::InterruptLYMatching)
            {
                self.stat_interrupt = true;
            }
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn mode_1<R: Renderer>(&mut self, renderer: &mut R, time: u64) {
        self.registers.stat.set_flag_value(LcdStatusFlag::Mode, 0x1);
        renderer.present();

        self.vertical_blanking_interrupt = true;

        if self
            .registers
            .stat
            .read_flag(LcdStatusFlag::InterruptMode01)
        {
            self.stat_interrupt = true;
        }

        self.scheduler
            .schedule(time + 4560, LcdControllerEvent::Mode2);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn enable(&mut self, time: u64) {
        assert!(!self.enabled);

        self.enabled = true;
        self.schedule_initial_events(time);
        self.update_ly_match(time);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn disable(&mut self) {
        assert!(self.enabled);

        let mode = self.registers.stat.read_flag_value(LcdStatusFlag::Mode);

        if mode == 0x2 || mode == 0x3 {
            self.oam_data.release();
            self.unusable_memory.release();
        }

        if mode == 0x3 {
            self.character_data.release();
            self.background_display_data_1.release();
            self.background_display_data_2.release();
        }

        self.registers.stat.set_flag_value(LcdStatusFlag::Mode, 0x0);
        self.registers.ly.set_value(0);
        self.scheduler.drop_events();

        self.enabled = false;
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn check_enabled_state(&mut self, time: u64) {
        let lcdc_enabled = self.registers.lcdc.read_flag(LcdControlFlag::DisplayOn);
        if self.enabled != lcdc_enabled {
            if lcdc_enabled {
                self.enable(time);
            } else {
                self.disable();
            }
        }
    }

    /// Should be called periodically to drive the emulator.
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn tick(&mut self, time: u64) {
        self.check_enabled_state(time);
    }
}

#[cfg(test)]
mod tests;
