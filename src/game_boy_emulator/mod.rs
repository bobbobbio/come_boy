// Copyright 2017 Remi Bernotavicius

extern crate sdl2;

mod debugger;
mod disassembler;

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::ops::Range;
use std::rc::Rc;
use std::iter;

pub use game_boy_emulator::debugger::run_debugger;
use lr35902_emulator::{LR35902Emulator, LR35902Flag, Intel8080Register};
use emulator_common::{MemoryAccessor, MemoryDescription};

pub use game_boy_emulator::disassembler::disassemble_game_boy_rom;

#[cfg(test)]
use util::super_fast_hash;

/*
 *  __  __
 * |  \/  | ___ _ __ ___   ___  _ __ _   _
 * | |\/| |/ _ \ '_ ` _ \ / _ \| '__| | | |
 * | |  | |  __/ | | | | | (_) | |  | |_| |
 * |_|  |_|\___|_| |_| |_|\___/|_|   \__, |
 *                                   |___/
 *   ____            _             _ _
 *  / ___|___  _ __ | |_ _ __ ___ | | | ___ _ __
 * | |   / _ \| '_ \| __| '__/ _ \| | |/ _ \ '__|
 * | |__| (_) | | | | |_| | | (_) | | |  __/ |
 *  \____\___/|_| |_|\__|_|  \___/|_|_|\___|_|
 */

struct GameBoyMemoryMap {
    memory_map: BTreeMap<u16, MemoryChunk>,
}

impl<'a> GameBoyMemoryMap {
    fn new() -> GameBoyMemoryMap
    {
        return GameBoyMemoryMap {
            memory_map: BTreeMap::new(),
        };
    }

    fn map_chunk(&mut self, address: u16, mut chunk: MemoryChunk)
    {
        // Assert the chunk we are mapping doesn't overlap with any existing chunks
        match self.memory_map.range(..address).last() {
            Some((&key, value)) => assert!(address < key || address >= key + value.len()),
            None => (),
        }

        match self.memory_map.range(address..).next() {
            Some((&key, _)) => assert!(address + chunk.len() <= key),
            None => (),
        }

        if address > 0 {
            assert!(chunk.len() <= (0xFFFF - address) + 1);
        }

        self.memory_map.insert(address, chunk.clone());
    }

    fn get_chunk_for_address(&self, address: u16) -> Option<(&u16, &MemoryChunk)>
    {
        if address == 0xFFFF {
            self.memory_map.iter().last()
        } else {
            self.memory_map.range(..address + 1).last()
        }
    }

    fn get_chunk_for_address_mut(&mut self, address: u16) -> Option<(&u16, &mut MemoryChunk)>
    {
        if address == 0xFFFF {
            self.memory_map.iter_mut().last()
        } else {
            self.memory_map.range_mut(..address + 1).last()
        }
    }

    // XXX this isn't used, but might be useful.
    #[allow(dead_code)]
    fn get_address_for_chunk(&self, c: &MemoryChunk) -> Option<u16>
    {
        for (&address, chunk) in &self.memory_map {
            if chunk.ptr_eq(c) {
                return Some(address);
            }
        }

        return None;
    }
}

impl<'a> MemoryAccessor for GameBoyMemoryMap {
    fn read_memory(&self, address: u16) -> u8
    {
        match self.get_chunk_for_address(address) {
            None => 0xFF,
            Some((key, ref chunk)) => {
                if address - key >= chunk.len() {
                    0xFF
                } else {
                    chunk.read_value(address - key)
                }
            },
        }
    }

    fn set_memory(&mut self, address: u16, value: u8)
    {
        match self.get_chunk_for_address_mut(address) {
            None => { },
            Some((key, chunk)) => {
                if address - key < chunk.len() {
                    chunk.set_value(address - key, value);
                }
            }
        };
    }

    fn describe_address(&self, _address: u16) -> MemoryDescription
    {
        return MemoryDescription::Instruction;
    }
}

struct MemoryChunk {
    value: Rc<RefCell<Vec<u8>>>
}

impl MemoryChunk {
    fn new(v: Vec<u8>) -> MemoryChunk
    {
        assert!(v.len() > 0);
        MemoryChunk {
            value: Rc::new(RefCell::new(v))
        }
    }

    fn set_value(&mut self, address: u16, value: u8)
    {
        (*self.value.borrow_mut())[address as usize] = value;
    }

    fn read_value(&self, address: u16) -> u8
    {
        (*self.value.borrow())[address as usize]
    }

    fn clone(&mut self) -> MemoryChunk
    {
        MemoryChunk {
            value: self.value.clone()
        }
    }

    fn len(&self) -> u16
    {
        (*self.value.borrow()).len() as u16
    }

    fn from_range(range: Range<u16>) -> MemoryChunk
    {
        let mut v = Vec::<u8>::new();
        v.resize(range.len(), 0);
        return MemoryChunk::new(v);
    }

    fn clone_from_slice(&mut self, slice: &[u8])
    {
        (*self.value.borrow_mut()).clone_from_slice(slice);
    }

    fn ptr_eq(&self, other: &MemoryChunk) -> bool
    {
        Rc::<RefCell<Vec<u8>>>::ptr_eq(&self.value, &other.value)
    }
}

struct MemoryChunkIterator<'a> {
    chunk: &'a MemoryChunk,
    current: u16,
}

impl<'a> Iterator for MemoryChunkIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8>
    {
        if self.current < self.chunk.len() {
            let mem = self.chunk.read_value(self.current);
            self.current += 1;
            return Some(mem);
        } else {
            return None;
        }
    }
}

impl<'a> MemoryChunkIterator<'a> {
    fn new(chunk: &'a MemoryChunk) -> MemoryChunkIterator
    {
        return MemoryChunkIterator {
            chunk: chunk,
            current: 0,
        };
    }
}

#[test]
#[should_panic]
fn overlapping_inside_chunk()
{
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(12, MemoryChunk::new(vec![1, 2, 3]));
    mm.map_chunk(13, MemoryChunk::new(vec![1]));
}

#[test]
#[should_panic]
fn overlapping_left_side_chunk()
{
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(12, MemoryChunk::new(vec![1, 2, 3]));
    mm.map_chunk(10, MemoryChunk::new(vec![1, 2, 3]));
}

#[test]
#[should_panic]
fn overlapping_right_side_chunk()
{
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(12, MemoryChunk::new(vec![1, 2, 3]));
    mm.map_chunk(14, MemoryChunk::new(vec![1, 2, 3]));
}

#[test]
fn non_overlapping_left_chunk()
{
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(3, MemoryChunk::new(vec![1, 2, 3]));
    mm.map_chunk(0, MemoryChunk::new(vec![1, 2, 3]));
}

#[test]
fn non_overlapping_right_chunk()
{
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(12, MemoryChunk::new(vec![1, 2, 3]));
    mm.map_chunk(15, MemoryChunk::new(vec![1, 2, 3]));
}

#[test]
#[should_panic]
fn mapping_past_end_of_range()
{
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(0xFFFF, MemoryChunk::new(vec![1, 2, 3]));
}

#[test]
fn accessing_unmapped_region()
{
    let mm = GameBoyMemoryMap::new();
    assert_eq!(mm.read_memory(24), 0xff);
}

#[test]
fn accessing_unmapped_region_with_region_mapped()
{
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(10, MemoryChunk::new(vec![9, 8, 7]));
    assert_eq!(mm.read_memory(24), 0xff);
}

#[test]
fn setting_unmapped_region()
{
    let mut mm = GameBoyMemoryMap::new();
    mm.set_memory(24, 99);
    assert_eq!(mm.read_memory(24), 0xFF);
}

#[test]
fn setting_unmapped_region_with_region_mapped()
{
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(10, MemoryChunk::new(vec![9, 8, 7]));
    mm.set_memory(24, 99);
    assert_eq!(mm.read_memory(24), 0xFF);
}

#[test]
fn accessing_mapped_region()
{
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(10, MemoryChunk::new(vec![9, 8, 7]));
    assert_eq!(mm.read_memory(10), 9);
    assert_eq!(mm.read_memory(11), 8);
    assert_eq!(mm.read_memory(12), 7);
}

#[test]
fn accessing_end_of_address_range()
{
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(0xFFFF, MemoryChunk::new(vec![9]));
    assert_eq!(mm.read_memory(0xFFFF), 9);
}

#[test]
fn setting_end_of_address_range()
{
    let mut mm = GameBoyMemoryMap::new();
    let mut chunk = MemoryChunk::new(vec![9]);
    mm.map_chunk(0xFFFF, chunk.clone());
    mm.set_memory(0xFFFF, 99);
    assert_eq!(chunk.read_value(0), 99);
}

#[test]
fn setting_mapped_region()
{
    let mut mm = GameBoyMemoryMap::new();
    let mut chunk = MemoryChunk::new(vec![9, 8, 7]);
    mm.map_chunk(10, chunk.clone());
    mm.set_memory(11, 88);
    assert_eq!(chunk.read_value(1), 88);
}

#[test]
fn memory_chunk_from_range()
{
    let mm = MemoryChunk::from_range(0..10);
    assert_eq!(mm.len(), 10);
}

/*  _     ____ ____   ____            _             _ _
 * | |   / ___|  _ \ / ___|___  _ __ | |_ _ __ ___ | | | ___ _ __
 * | |  | |   | | | | |   / _ \| '_ \| __| '__/ _ \| | |/ _ \ '__|
 * | |__| |___| |_| | |__| (_) | | | | |_| | | (_) | | |  __/ |
 * |_____\____|____/ \____\___/|_| |_|\__|_|  \___/|_|_|\___|_|
 */

const VERTICAL_BLANKING_INTERRUPT_ADDRESS : u16 = 0x0040;
const LCDCSTATUS_INTERRUPT_ADDRESS : u16 = 0x0048;
const TIMER_INTERRUPT_ADDRESS : u16 = 0x0050;

const CHARACTER_DATA: Range<u16> = Range { start: 0x8000, end: 0x9800 };
const CHARACTER_DATA_1: Range<u16> = Range { start: 0x0, end: 0x1000};
const CHARACTER_DATA_2: Range<u16> = Range { start: 0x800, end: 0x1800};
const BACKGROUND_DISPLAY_DATA_1: Range<u16> = Range { start: 0x9800, end: 0x9C00 };
const BACKGROUND_DISPLAY_DATA_2: Range<u16> = Range { start: 0x9C00, end: 0xA000 };
const OAM_DATA: Range<u16> = Range { start: 0xFE00, end: 0xFEA0 };

/*
 * Number of pixels (both horizontal and vertical) on the screen per gameboy pixel.
 */
const PIXEL_SCALE: u32 = 4;

const CHARACTER_SIZE: u8 = 8;
const CHARACTER_AREA_SIZE: u16 = 32;

struct LCDController<'a> {
    renderer: Option<sdl2::render::Renderer<'a>>,
    event_pump: Option<sdl2::EventPump>,
    pub crash_message: Option<String>,
    character_data: MemoryChunk,
    background_display_data_1: MemoryChunk,
    background_display_data_2: MemoryChunk,
    oam_data: MemoryChunk,

    lcdc: GameBoyRegister,
    stat: GameBoyRegister,
    scy: GameBoyRegister,
    scx: GameBoyRegister,
    ly: GameBoyRegister,
    lyc: GameBoyRegister,
    dma: GameBoyRegister,
    bgp: GameBoyRegister,
    obp0: GameBoyRegister,
    obp1: GameBoyRegister,
    wy: GameBoyRegister,
    wx: GameBoyRegister,
}

#[derive(Debug,Clone,Copy,PartialEq)]
enum LCDBGShade {
    Shade0 = 0x0,
    Shade1 = 0x1,
    Shade2 = 0x2,
    Shade3 = 0x3,
}

fn color_for_shade(shade: LCDBGShade) -> sdl2::pixels::Color
{
    match shade {
        LCDBGShade::Shade0 => sdl2::pixels::Color::RGB(255, 255, 255),
        LCDBGShade::Shade1 => sdl2::pixels::Color::RGB(105, 150, 150),
        LCDBGShade::Shade2 => sdl2::pixels::Color::RGB(50, 50, 50),
        LCDBGShade::Shade3 => sdl2::pixels::Color::RGB(0, 0, 0),
    }
}

#[derive(Debug,Clone,Copy,PartialEq)]
enum LCDControlFlag {
                                     // 76543210
    OperationStop =                   0b10000000,
    WindowCodeAreaSelection =         0b01000000,
    WindowingOn =                     0b00100000,
    BGCharacterDataSelection =        0b00010000,
    BGCodeAreaSelection =             0b00001000,
    ObjectBlockCompositionSelection = 0b00000100,
    ObjectOn =                        0b00000010,
    BGDisplayOn =                     0b00000001,
}

#[derive(Debug,Clone,Copy,PartialEq)]
enum LCDStatusFlag {
                         // 76543210
    InterruptLYMatching = 0b10000000,
    InterruptMode10 =     0b01000000,
    InterruptMode01 =     0b00100000,
    InterruptMode00 =     0b00010000,
    LYMatch =             0b00001000,
    Mode =                0b00000011,
}

#[derive(Debug,Clone,Copy,PartialEq)]
enum InterruptFlag {
                      // 76543210
    VerticalBlanking = 0b00000001,
    LCDSTAT          = 0b00000010,
    Timer            = 0b00000100,
    /*
    Serial           = 0b00001000,
    Joypad           = 0b00010000,
    */
}

#[derive(Debug,Clone,Copy,PartialEq)]
struct LCDObject {
    y_coordinate: u8,
    x_coordinate: u8,
    character_code: u8,
    flags: u8
}

/*
#[derive(Debug,Clone,Copy,PartialEq)]
enum LCDObjectAttributeFlag {
                     // 76543210
    DisplayPriority = 0b10000000,
    VerticalFlip =    0b01000000,
    HorizantalFlip =  0b00100000,
    Palette =         0b00010000,
}

impl LCDObject {
    fn read_flag(&self, flag: LCDObjectAttributeFlag) -> bool
    {
        self.flags & flag as u8 == flag as u8
    }
}
*/

struct LCDObjectIterator<'a> {
    chunk_iterator: iter::Peekable<MemoryChunkIterator<'a>>
}

impl<'a> Iterator for LCDObjectIterator<'a> {
    type Item = LCDObject;

    fn next(&mut self) -> Option<LCDObject>
    {
        if self.chunk_iterator.peek() == None {
            return None
        } else {
            let lcd_object = LCDObject {
                y_coordinate: self.chunk_iterator.next().unwrap(),
                x_coordinate: self.chunk_iterator.next().unwrap(),
                character_code: self.chunk_iterator.next().unwrap(),
                flags: self.chunk_iterator.next().unwrap()
            };
            return Some(lcd_object);
        }
    }
}

impl<'a> LCDObjectIterator<'a> {
    fn new(chunk: &'a MemoryChunk) -> LCDObjectIterator<'a>
    {
        LCDObjectIterator {
            chunk_iterator: MemoryChunkIterator::new(chunk).peekable()
        }
    }
}

struct LCDDotData {
    data: [LCDBGShade; 64]
}

impl LCDDotData {
    fn new() -> LCDDotData {
        LCDDotData {
            data: [LCDBGShade::Shade0; 64]
        }
    }

    fn draw(&self, renderer: &mut sdl2::render::Renderer, x: i32, y: i32)
    {
        for (p, shade) in self.data.iter().enumerate() {
            let (offset_x, offset_y) = ((p as u8 % CHARACTER_SIZE), (p as u8 / CHARACTER_SIZE));
            let rect = sdl2::rect::Rect::new(
                (x + offset_x as i32) * PIXEL_SCALE as i32,
                (y + offset_y as i32) * PIXEL_SCALE as i32, PIXEL_SCALE, PIXEL_SCALE);
            let color = color_for_shade(*shade);
            renderer.set_draw_color(color);
            renderer.fill_rect(rect).unwrap();
        }
    }
}

impl<'a> LCDController<'a> {
    fn new() -> LCDController<'a>
    {
        LCDController {
            renderer: None,
            event_pump: None,
            crash_message: None,
            character_data: MemoryChunk::from_range(CHARACTER_DATA),
            background_display_data_1: MemoryChunk::from_range(BACKGROUND_DISPLAY_DATA_1),
            background_display_data_2: MemoryChunk::from_range(BACKGROUND_DISPLAY_DATA_2),
            oam_data: MemoryChunk::from_range(OAM_DATA),
            lcdc: GameBoyRegister::new(),
            stat: GameBoyRegister::new(),
            scy: GameBoyRegister::new(),
            scx: GameBoyRegister::new(),
            ly: GameBoyRegister::new(),
            lyc: GameBoyRegister::new(),
            dma: GameBoyRegister::new(),
            bgp: GameBoyRegister::new(),
            obp0: GameBoyRegister::new(),
            obp1: GameBoyRegister::new(),
            wy: GameBoyRegister::new(),
            wx: GameBoyRegister::new(),
        }
    }

    fn start_rendering(&mut self)
    {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        let window = video_subsystem.window("come boy", 160 * PIXEL_SCALE, 144 * PIXEL_SCALE)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut renderer = window.renderer().build().unwrap();
        renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        renderer.clear();
        renderer.present();

        self.renderer = Some(renderer);
        self.event_pump = Some(event_pump);
    }

    fn read_dot_data(&self, character_code: u8) -> LCDDotData
    {
        let mut dot_data = LCDDotData::new();

        let location = if self.read_lcd_control_flag(LCDControlFlag::BGCharacterDataSelection) {
            CHARACTER_DATA_1.start
        } else {
            CHARACTER_DATA_2.start
        } as usize + character_code as usize * 16;

        let mut iter = MemoryChunkIterator::new(
            &self.character_data).skip(location).take(16).peekable();

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
                    _ => panic!("")
                };
                i += 1;
            }
        }
        assert_eq!(i, 64);

        return dot_data;
    }

    fn read_lcd_control_flag(&self, flag: LCDControlFlag) -> bool
    {
        self.lcdc.read_value() & flag as u8 == flag as u8
    }

    fn set_lcd_control_flag(&mut self, flag: LCDControlFlag, value: bool)
    {
        let old_value = self.lcdc.read_value();
        if value {
            self.lcdc.set_value(old_value | flag as u8);
        } else {
            self.lcdc.set_value(old_value & !(flag as u8));
        }
    }

    /*
    fn read_lcd_status_flag(&self, flag: LCDStatusFlag) -> bool
    {
        self.stat.read_value() & flag as u8 == flag as u8
    }
    */

    fn set_lcd_status_flag(&mut self, flag: LCDStatusFlag, value: bool)
    {
        // Mode is a four-value flag
        assert!(flag != LCDStatusFlag::Mode);

        let old_value = self.stat.read_value();
        if value {
            self.stat.set_value(old_value | flag as u8);
        } else {
            self.stat.set_value(old_value & !(flag as u8));
        }
    }

    fn set_lcd_status_mode(&mut self, value: u8)
    {
        let stat = self.stat.read_value() & !(LCDStatusFlag::Mode as u8);
        self.stat.set_value(stat | value);
    }

    fn initialize_flags(&mut self)
    {
        self.set_lcd_control_flag(LCDControlFlag::OperationStop, true);
        self.set_lcd_control_flag(LCDControlFlag::BGCharacterDataSelection, true);
        self.set_lcd_control_flag(LCDControlFlag::BGDisplayOn, true);
        self.bgp.set_value(0xFC);

        self.set_lcd_status_flag(LCDStatusFlag::InterruptLYMatching, true);
        self.set_lcd_status_mode(0x1);
    }

    fn get_scroll_origin_relative_to_lcd(&self) -> (i32, i32)
    {
        let mut x = self.scx.read_value() as i32 * -1;
        let mut y = self.scy.read_value() as i32 * -1;

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

    fn get_window_origin_relative_to_lcd(&self) -> (i32, i32)
    {
        let x = self.wx.read_value() as i32 * -1;
        let y = self.wy.read_value() as i32 * -1;

        return (x, y);
    }

    fn crashed(&self) -> bool
    {
        self.crash_message.is_some()
    }

    fn draw_one_line(&mut self)
    {
        /*
         * Update the LY register which represents the line being draw currently.
         */
        let ly = self.ly.read_value();
        if ly == 153 {
            self.ly.set_value(0);
        } else {
            self.ly.set_value(ly + 1);
        }

        /*
         * We don't actually draw anything on the screen until LY gets to zero.
         *
         * On a real gameboy it draws one line at a time, but drawing the whole screen at a time
         * prevents ghosting effects.
         */
        if ly != 0 {
            return;
        }

        if self.renderer.is_none() {
            return;
        }

        /*
         * Other than checking to see if the user has closed the window, for some reason if we do
         * not poll the event_pump the screen will not draw properly.
         */
        for event in self.event_pump.as_mut().unwrap().poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    self.crash_message = Some(String::from("Screen Closed"));
                    return;
                },
                _ => {}
            }
        }

        self.renderer.as_mut().unwrap().set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        self.renderer.as_mut().unwrap().clear();

        let (scroll_x, scroll_y) = self.get_scroll_origin_relative_to_lcd();

        let iter = match self.read_lcd_control_flag(LCDControlFlag::BGCodeAreaSelection) {
            false => MemoryChunkIterator::new(&self.background_display_data_1),
            true => MemoryChunkIterator::new(&self.background_display_data_2),
        }.enumerate();

        for (c, character_code) in iter {
            let character_data = self.read_dot_data(character_code);
            let character_x = scroll_x
                + ((c as u16 % CHARACTER_AREA_SIZE) as u8 * CHARACTER_SIZE) as i32;
            let character_y = scroll_y
                + ((c as u16 / CHARACTER_AREA_SIZE) as u8 * CHARACTER_SIZE) as i32;
            character_data.draw(self.renderer.as_mut().unwrap(), character_x, character_y);
        }

        let (window_x, window_y) = self.get_window_origin_relative_to_lcd();
        let iter = LCDObjectIterator::new(&self.oam_data);
        for object in iter {
            let character_data = self.read_dot_data(object.character_code);
            character_data.draw(
                self.renderer.as_mut().unwrap(),
                window_x + object.x_coordinate as i32,
                window_y + object.y_coordinate as i32);
        }

        self.renderer.as_mut().unwrap().present();
    }

    fn process_interrupts(&mut self, interrupt_flag: &mut GameBoyRegister)
    {
        let ly = self.ly.read_value();

        let interrupt_flag_value = interrupt_flag.read_value();

        // Vertical blanking starts when ly == 144
        if ly == 144 {
            interrupt_flag.set_value(interrupt_flag_value | InterruptFlag::VerticalBlanking as u8);
        }

        if ly == self.ly.read_value() {
            interrupt_flag.set_value(interrupt_flag_value | InterruptFlag::LCDSTAT as u8);
            self.set_lcd_status_flag(LCDStatusFlag::InterruptLYMatching, true);
            self.set_lcd_status_flag(LCDStatusFlag::LYMatch, true);
        }
    }
}

/*   ____                      ____              _____                 _       _
 *  / ___| __ _ _ __ ___   ___| __ )  ___  _   _| ____|_ __ ___  _   _| | __ _| |_ ___  _ __
 * | |  _ / _` | '_ ` _ \ / _ \  _ \ / _ \| | | |  _| | '_ ` _ \| | | | |/ _` | __/ _ \| '__|
 * | |_| | (_| | | | | | |  __/ |_) | (_) | |_| | |___| | | | | | |_| | | (_| | || (_) | |
 *  \____|\__,_|_| |_| |_|\___|____/ \___/ \__, |_____|_| |_| |_|\__,_|_|\__,_|\__\___/|_|
 *                                         |___/
 */

// const CARTRIDGE_RAM: Range<u16> = Range { start: 0xA000, end: 0xC000 };
// const UNUSABLE_MEMORY: Range<u16> = Range { start: 0xFEA0, end: 0xFF00 };
const IO_PORTS_A: Range<u16> = Range { start: 0xFF10, end: 0xFF40 };
const IO_PORTS_B: Range<u16> = Range { start: 0xFF4C, end: 0xFF80 };
const HIGH_RAM: Range<u16> = Range { start: 0xFF80, end: 0xFFFF };

struct GameBoyRegister {
    chunk: MemoryChunk,
}

impl GameBoyRegister {
    fn new() -> GameBoyRegister
    {
        GameBoyRegister {
            chunk: MemoryChunk::from_range(0..1),
        }
    }

    fn read_value(&self) -> u8
    {
        self.chunk.read_value(0)
    }

    fn set_value(&mut self, value: u8)
    {
        self.chunk.set_value(0, value)
    }

    fn add(&mut self, value: u8)
    {
        let old_value = self.read_value();
        self.set_value(old_value.wrapping_add(value));
    }

    // XXX this isn't used, but might be useful.
    #[allow(dead_code)]
    fn subtract(&mut self, value: u8)
    {
        let old_value = self.read_value();
        self.set_value(old_value.wrapping_sub(value));
    }
}

struct GameBoyEmulator<'a> {
    cpu: LR35902Emulator<GameBoyMemoryMap>,
    lcd_controller: LCDController<'a>,
    last_draw: u64,
    last_timer_tick: u64,
    io_ports_a: MemoryChunk,
    io_ports_b: MemoryChunk,
    high_ram: MemoryChunk,

    interrupt_flag: GameBoyRegister,
    interrupt_enable: GameBoyRegister,

    p1_joypad: GameBoyRegister,
    serial_transfer_data: GameBoyRegister,
    serial_transfer_control: GameBoyRegister,
    divider: GameBoyRegister,

    timer_counter: GameBoyRegister,
    timer_modulo: GameBoyRegister,
    timer_control: GameBoyRegister,
}

impl<'a> GameBoyEmulator<'a> {
    fn new() -> GameBoyEmulator<'a> {
        let mut e = GameBoyEmulator {
            cpu: LR35902Emulator::new(GameBoyMemoryMap::new()),
            lcd_controller: LCDController::new(),
            last_draw: 0,
            last_timer_tick: 0,
            io_ports_a: MemoryChunk::from_range(IO_PORTS_A),
            io_ports_b: MemoryChunk::from_range(IO_PORTS_B),
            high_ram: MemoryChunk::from_range(HIGH_RAM),
            interrupt_flag: GameBoyRegister::new(),
            interrupt_enable: GameBoyRegister::new(),
            p1_joypad: GameBoyRegister::new(),
            serial_transfer_data: GameBoyRegister::new(),
            serial_transfer_control: GameBoyRegister::new(),
            divider: GameBoyRegister::new(),
            timer_counter: GameBoyRegister::new(),
            timer_modulo: GameBoyRegister::new(),
            timer_control: GameBoyRegister::new(),
        };

        // Restart and interrupt vectors (unmapped) 0x0000 - 0x00FF

        // Rom (unmapped) 0x0100 - 0x97FF

        // Character data 0x8000 - 0x97FF
        e.cpu.memory_accessor.map_chunk(
            CHARACTER_DATA.start, e.lcd_controller.character_data.clone());

        // Background display data 0x9800 - 0x9FFF
        e.cpu.memory_accessor.map_chunk(
            BACKGROUND_DISPLAY_DATA_1.start, e.lcd_controller.background_display_data_1.clone());
        e.cpu.memory_accessor.map_chunk(
            BACKGROUND_DISPLAY_DATA_2.start, e.lcd_controller.background_display_data_2.clone());

        // Cartridge RAM (unmapped) 0xA000 - 0xBFFF

        // Internal RAM 0xC000 - 0xDFFF
        e.cpu.memory_accessor.map_chunk(0xC000, MemoryChunk::from_range(0..0x2000));

        // Echo RAM 0xE000 - 0xFDFF
        e.cpu.memory_accessor.map_chunk(0xE000, MemoryChunk::from_range(0..0x1E00));

        // OAM Data 0xFE00 - 0xFE9F
        e.cpu.memory_accessor.map_chunk(OAM_DATA.start, e.lcd_controller.oam_data.clone());

        // Unusable memory 0xFEA0 - 0xFEFF
        e.cpu.memory_accessor.map_chunk(0xFEA0, MemoryChunk::from_range(0..0x60));

        // Registers
        e.cpu.memory_accessor.map_chunk(0xFF00, e.p1_joypad.chunk.clone());
        e.cpu.memory_accessor.map_chunk(0xFF01, e.serial_transfer_data.chunk.clone());
        e.cpu.memory_accessor.map_chunk(0xFF02, e.serial_transfer_control.chunk.clone());

        e.cpu.memory_accessor.map_chunk(0xFF04, e.divider.chunk.clone());
        e.cpu.memory_accessor.map_chunk(0xFF05, e.timer_counter.chunk.clone());
        e.cpu.memory_accessor.map_chunk(0xFF06, e.timer_modulo.chunk.clone());
        e.cpu.memory_accessor.map_chunk(0xFF07, e.timer_control.chunk.clone());

        e.cpu.memory_accessor.map_chunk(0xFF0F, e.interrupt_flag.chunk.clone());

        // Other IO Registers 0xFF10 - 0xFF3F
        e.cpu.memory_accessor.map_chunk(IO_PORTS_A.start, e.io_ports_a.clone());

        // LCD Registers 0xFF40 - 0xFF4B
        e.cpu.memory_accessor.map_chunk(0xFF40, e.lcd_controller.lcdc.chunk.clone());
        e.cpu.memory_accessor.map_chunk(0xFF41, e.lcd_controller.stat.chunk.clone());
        e.cpu.memory_accessor.map_chunk(0xFF42, e.lcd_controller.scy.chunk.clone());
        e.cpu.memory_accessor.map_chunk(0xFF43, e.lcd_controller.scx.chunk.clone());
        e.cpu.memory_accessor.map_chunk(0xFF44, e.lcd_controller.ly.chunk.clone());
        e.cpu.memory_accessor.map_chunk(0xFF45, e.lcd_controller.lyc.chunk.clone());
        e.cpu.memory_accessor.map_chunk(0xFF46, e.lcd_controller.dma.chunk.clone());
        e.cpu.memory_accessor.map_chunk(0xFF47, e.lcd_controller.bgp.chunk.clone());
        e.cpu.memory_accessor.map_chunk(0xFF48, e.lcd_controller.obp0.chunk.clone());
        e.cpu.memory_accessor.map_chunk(0xFF49, e.lcd_controller.obp1.chunk.clone());
        e.cpu.memory_accessor.map_chunk(0xFF4A, e.lcd_controller.wy.chunk.clone());
        e.cpu.memory_accessor.map_chunk(0xFF4B, e.lcd_controller.wx.chunk.clone());

        // Other IO Registers 0xFF4C - 0xFF7F
        e.cpu.memory_accessor.map_chunk(IO_PORTS_B.start, e.io_ports_b.clone());

        // High RAM 0xFF80 - 0xFFFE
        e.cpu.memory_accessor.map_chunk(HIGH_RAM.start, e.high_ram.clone());

        // interrupt enable register
        e.cpu.memory_accessor.map_chunk(0xFFFF, e.interrupt_enable.chunk.clone());

        e.lcd_controller.initialize_flags();

        e.set_state_post_bios();

        return e;
    }

    fn load_rom(&mut self, rom: &[u8])
    {
        self.cpu.memory_accessor.map_chunk(0, MemoryChunk::new(rom.to_vec()));
    }

    fn crashed(&self) -> Option<&String>
    {
        if self.cpu.crashed() {
            return self.cpu.crash_message.as_ref()
        } else if self.lcd_controller.crashed() {
            return self.lcd_controller.crash_message.as_ref()
        }

        None
    }

    fn tick(&mut self)
    {
        self.cpu.run_one_instruction();

        let current_clock = self.cpu.elapsed_cycles;

        if current_clock - self.last_draw > 40 {
            self.lcd_controller.draw_one_line();
            self.lcd_controller.process_interrupts(&mut self.interrupt_flag);
            self.last_draw = current_clock;
        }

        if current_clock - self.last_timer_tick > 16384 {
            self.tick_timer();
            self.last_timer_tick = current_clock;
        }

        if self.cpu.get_interrupts_enabled() {
            self.handle_interrupts();
        }
    }

    fn handle_interrupts(&mut self)
    {
        let interrupt_flag_value = self.interrupt_flag.read_value();
        let interrupt_enable_value = self.interrupt_enable.read_value();

        if interrupt_flag_value & InterruptFlag::VerticalBlanking as u8 != 0 &&
            interrupt_enable_value & InterruptFlag::VerticalBlanking as u8 != 0 {
            self.interrupt_flag.set_value(
                interrupt_flag_value & !(InterruptFlag::VerticalBlanking as u8));
            self.cpu.interrupt(VERTICAL_BLANKING_INTERRUPT_ADDRESS);
        }

        if interrupt_flag_value & InterruptFlag::LCDSTAT as u8 != 0 &&
            interrupt_enable_value & InterruptFlag::LCDSTAT as u8 != 0 {
            self.cpu.interrupt(LCDCSTATUS_INTERRUPT_ADDRESS);
        }

        if interrupt_flag_value & InterruptFlag::Timer as u8 != 0 &&
            interrupt_enable_value & InterruptFlag::Timer as u8 != 0 {
            self.cpu.interrupt(TIMER_INTERRUPT_ADDRESS);
        }
    }

    fn tick_timer(&mut self)
    {
        let interrupt_flag_value = self.interrupt_flag.read_value();

        if self.timer_counter.read_value() == 0xff {
            let timer_modulo_value = self.timer_modulo.read_value();
            self.timer_counter.set_value(timer_modulo_value);
            self.interrupt_flag.set_value(interrupt_flag_value | InterruptFlag::Timer as u8);
        } else {
            self.timer_counter.add(0x1);
        }
    }

    fn set_state_post_bios(&mut self)
    {
        /*
         * After running the BIOS (the part of the gameboy that shows the logo) the cpu is left in
         * a very certain state. Since this is always the case, certain games may rely on this fact
         * (and indeed often times do.)
         */
        self.cpu.set_register(Intel8080Register::A, 0x11);
        self.cpu.set_register(Intel8080Register::B, 0x0);
        self.cpu.set_register(Intel8080Register::C, 0x13);
        self.cpu.set_register(Intel8080Register::D, 0x0);
        self.cpu.set_register(Intel8080Register::E, 0xD8);
        self.cpu.set_register(Intel8080Register::H, 0x01);
        self.cpu.set_register(Intel8080Register::L, 0x4D);
        self.cpu.set_flag(LR35902Flag::Carry, true);
        self.cpu.set_flag(LR35902Flag::HalfCarry, true);
        self.cpu.set_flag(LR35902Flag::Subtract, false);
        self.cpu.set_flag(LR35902Flag::Zero, true);

        self.p1_joypad.set_value(0xcf);
        self.serial_transfer_data.set_value(0x0);
        self.serial_transfer_control.set_value(0x7c);

        self.divider.set_value(0x69);
        self.timer_counter.set_value(0x0);
        self.timer_modulo.set_value(0x0);
        self.timer_control.set_value(0xf8);

        self.interrupt_flag.set_value(0xe1);

        // Initialize io ports
        let io_ports_a = [
            /* 10 - 17 */ 0x80u8, 0xbfu8, 0xf3u8, 0xffu8, 0xbfu8, 0xffu8, 0x3fu8, 0x00u8,
            /* 18 - 1f */ 0xffu8, 0xbfu8, 0x7fu8, 0xffu8, 0x9fu8, 0xffu8, 0xbfu8, 0xffu8,
            /* 20 - 27 */ 0xffu8, 0x00u8, 0x00u8, 0xbfu8, 0x77u8, 0xf3u8, 0xf1u8, 0xffu8,
            /* 28 - 2f */ 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8,
            /* 30 - 37 */ 0x00u8, 0xffu8, 0x00u8, 0xffu8, 0x00u8, 0xffu8, 0x00u8, 0xffu8,
            /* 38 - 3f */ 0x00u8, 0xffu8, 0x00u8, 0xffu8, 0x00u8, 0xffu8, 0x00u8, 0xffu8
        ];
        self.io_ports_a.clone_from_slice(&io_ports_a);

        /* 40 - 4B LCD Controller */

        let io_ports_b = [
            /* 4c - 4f */ 0xffu8, 0x7eu8, 0xffu8, 0x00u8,
            /* 50 - 57 */ 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0x3eu8, 0xffu8,
            /* 58 - 5f */ 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8,
            /* 60 - 67 */ 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8,
            /* 68 - 6f */ 0xc0u8, 0xffu8, 0xc1u8, 0x00u8, 0xfeu8, 0xffu8, 0xffu8, 0xffu8,
            /* 70 - 77 */ 0xf8u8, 0xffu8, 0x00u8, 0x00u8, 0xfeu8, 0x8fu8, 0x00u8, 0x00u8,
            /* 78 - 7f */ 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8
        ];
        self.io_ports_b.clone_from_slice(&io_ports_b);

        let high_ram = [
            0xceu8, 0xedu8, 0x66u8, 0x66u8, 0xccu8, 0x0du8, 0x00u8, 0x0bu8, 0x03u8, 0x73u8, 0x00u8,
            0x83u8, 0x00u8, 0x0cu8, 0x00u8, 0x0du8, 0x00u8, 0x08u8, 0x11u8, 0x1fu8, 0x88u8, 0x89u8,
            0x00u8, 0x0eu8, 0xdcu8, 0xccu8, 0x6eu8, 0xe6u8, 0xddu8, 0xddu8, 0xd9u8, 0x99u8, 0xbbu8,
            0xbbu8, 0x67u8, 0x63u8, 0x6eu8, 0x0eu8, 0xecu8, 0xccu8, 0xddu8, 0xdcu8, 0x99u8, 0x9fu8,
            0xbbu8, 0xb9u8, 0x33u8, 0x3eu8, 0x45u8, 0xecu8, 0x52u8, 0xfau8, 0x08u8, 0xb7u8, 0x07u8,
            0x5du8, 0x01u8, 0xfdu8, 0xc0u8, 0xffu8, 0x08u8, 0xfcu8, 0x00u8, 0xe5u8, 0x0bu8, 0xf8u8,
            0xc2u8, 0xceu8, 0xf4u8, 0xf9u8, 0x0fu8, 0x7fu8, 0x45u8, 0x6du8, 0x3du8, 0xfeu8, 0x46u8,
            0x97u8, 0x33u8, 0x5eu8, 0x08u8, 0xefu8, 0xf1u8, 0xffu8, 0x86u8, 0x83u8, 0x24u8, 0x74u8,
            0x12u8, 0xfcu8, 0x00u8, 0x9fu8, 0xb4u8, 0xb7u8, 0x06u8, 0xd5u8, 0xd0u8, 0x7au8, 0x00u8,
            0x9eu8, 0x04u8, 0x5fu8, 0x41u8, 0x2fu8, 0x1du8, 0x77u8, 0x36u8, 0x75u8, 0x81u8, 0xaau8,
            0x70u8, 0x3au8, 0x98u8, 0xd1u8, 0x71u8, 0x02u8, 0x4du8, 0x01u8, 0xc1u8, 0xffu8, 0x0du8,
            0x00u8, 0xd3u8, 0x05u8, 0xf9u8, 0x00u8, 0x0bu8
        ];
        self.high_ram.clone_from_slice(&high_ram);

        self.interrupt_enable.set_value(0x0);
    }

    fn run(&mut self)
    {
        self.lcd_controller.start_rendering();
        while self.crashed().is_none() {
            self.tick();
        }
        if self.cpu.crashed() {
            println!("Emulator crashed: {}", self.cpu.crash_message.as_ref().unwrap());
        }
    }

    #[cfg(test)]
    fn hash(&self) -> u32
    {
        // Read up all of memory
        let mut mem = [0u8; 0x10000 + 15];
        for i in 0..0x10000 {
            mem[i] = self.cpu.memory_accessor.read_memory(i as u16);
        }

        mem[0x10000 + 0] = self.cpu.read_register(Intel8080Register::A);
        mem[0x10000 + 1] = self.cpu.read_register(Intel8080Register::B);
        mem[0x10000 + 2] = self.cpu.read_register(Intel8080Register::C);
        mem[0x10000 + 3] = self.cpu.read_register(Intel8080Register::D);
        mem[0x10000 + 4] = self.cpu.read_register(Intel8080Register::E);
        mem[0x10000 + 5] = self.cpu.read_register(Intel8080Register::H);
        mem[0x10000 + 6] = self.cpu.read_register(Intel8080Register::L);
        mem[0x10000 + 7] = (self.cpu.read_program_counter() >> 8) as u8;
        mem[0x10000 + 8] = (self.cpu.read_program_counter() & 0xFF) as u8;
        mem[0x10000 + 9] = (self.cpu.read_register_pair(Intel8080Register::SP) >> 8) as u8;
        mem[0x10000 + 10] = (self.cpu.read_register_pair(Intel8080Register::SP) & 0xFF) as u8;
        mem[0x10000 + 11] = if self.cpu.read_flag(LR35902Flag::Zero) { 1 } else { 0 };
        mem[0x10000 + 12] = if self.cpu.read_flag(LR35902Flag::Subtract) { 1 } else { 0 };
        mem[0x10000 + 13] = if self.cpu.read_flag(LR35902Flag::HalfCarry) { 1 } else { 0 };
        mem[0x10000 + 14] = if self.cpu.read_flag(LR35902Flag::Carry) { 1 } else { 0 };

        return super_fast_hash(&mem);
    }
}

#[test]
fn initial_state_test()
{
    let e = GameBoyEmulator::new();

    // Lock down the initial state.
    assert_eq!(e.hash(), 2735489010);
}

/*  ____  _                         _____         _     ____   ___  __  __
 * | __ )| | __ _ _ __ __ _  __ _  |_   _|__  ___| |_  |  _ \ / _ \|  \/  |___
 * |  _ \| |/ _` | '__/ _` |/ _` |   | |/ _ \/ __| __| | |_) | | | | |\/| / __|
 * | |_) | | (_| | | | (_| | (_| |   | |  __/\__ \ |_  |  _ <| |_| | |  | \__ \
 * |____/|_|\__,_|_|  \__, |\__, |   |_|\___||___/\__| |_| \_\\___/|_|  |_|___/
 *                    |___/ |___/
 *
 */

#[cfg(test)]
use lr35902_emulator::{read_blargg_test_rom, run_blargg_test_rom};

#[test]
#[ignore]
fn blargg_test_rom_cpu_instrs_2_interrupts()
{
    let mut e = GameBoyEmulator::new();
    e.load_rom(&read_blargg_test_rom("cpu_instrs/individual/02-interrupts.gb"));
    run_blargg_test_rom(&mut e.cpu, 0xc7f4);
}

pub fn run_emulator(rom: &[u8])
{
    let mut e = GameBoyEmulator::new();
    e.load_rom(&rom);
    e.run();
}
