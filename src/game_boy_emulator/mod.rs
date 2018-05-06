// Copyright 2017 Remi Bernotavicius

extern crate sdl2;

mod debugger;
mod disassembler;
mod tandem;

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::{self, Write};
use std::iter;
use std::ops::Range;
use std::rc::Rc;

pub use self::debugger::run_debugger;
use emulator_common::{MemoryAccessor, MemoryDescription};
use lr35902_emulator::{Intel8080Register, LR35902Emulator, LR35902Flag};

pub use self::disassembler::disassemble_game_boy_rom;

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
    fn new() -> GameBoyMemoryMap {
        return GameBoyMemoryMap {
            memory_map: BTreeMap::new(),
        };
    }

    fn map_chunk(&mut self, address: u16, mut chunk: MemoryChunk) {
        // Assert the chunk we are mapping doesn't overlap with any existing chunks
        match self.memory_map.range(..address).last() {
            Some((&key, value)) => assert!(
                address < key || address >= key + value.len(),
                "Chunk overlaps existing chunk"
            ),
            None => (),
        }

        match self.memory_map.range(address..).next() {
            Some((&key, _)) => assert!(
                address + chunk.len() <= key,
                "Chunk overlaps existing chunk"
            ),
            None => (),
        }

        if address > 0 {
            assert!(
                chunk.len() <= (0xFFFF - address) + 1,
                "Chunk extends past end of address space"
            );
        }

        self.memory_map.insert(address, chunk.clone());
    }

    // XXX this isn't used, but might be useful.
    #[allow(dead_code)]
    fn unmap_chunk(&mut self, address: u16) {
        self.memory_map.remove(&address);
    }

    fn get_chunk_for_address(&self, address: u16) -> Option<(&u16, &MemoryChunk)> {
        if address == 0xFFFF {
            self.memory_map.iter().last()
        } else {
            self.memory_map.range(..address + 1).last()
        }
    }

    fn get_chunk_for_address_mut(&mut self, address: u16) -> Option<(&u16, &mut MemoryChunk)> {
        if address == 0xFFFF {
            self.memory_map.iter_mut().last()
        } else {
            self.memory_map.range_mut(..address + 1).last()
        }
    }

    // XXX this isn't used, but might be useful.
    #[allow(dead_code)]
    fn get_address_for_chunk(&self, c: &MemoryChunk) -> Option<u16> {
        for (&address, chunk) in &self.memory_map {
            if chunk.ptr_eq(c) {
                return Some(address);
            }
        }

        return None;
    }
}

impl<'a> MemoryAccessor for GameBoyMemoryMap {
    fn read_memory(&self, address: u16) -> u8 {
        match self.get_chunk_for_address(address) {
            None => 0xFF,
            Some((key, ref chunk)) => {
                if address - key >= chunk.len() {
                    0xFF
                } else {
                    chunk.read_value(address - key)
                }
            }
        }
    }

    fn set_memory(&mut self, address: u16, value: u8) {
        match self.get_chunk_for_address_mut(address) {
            None => {}
            Some((key, chunk)) => {
                if address - key < chunk.len() {
                    chunk.set_value(address - key, value);
                }
            }
        };
    }

    fn describe_address(&self, _address: u16) -> MemoryDescription {
        return MemoryDescription::Instruction;
    }
}

#[derive(Default)]
struct MemoryChunk {
    value: Rc<RefCell<(bool, Vec<u8>)>>,
    borrower: bool,
}

impl MemoryChunk {
    fn new(v: Vec<u8>) -> MemoryChunk {
        assert!(v.len() > 0);
        MemoryChunk {
            value: Rc::new(RefCell::new((false, v))),
            borrower: false,
        }
    }

    fn set_value(&mut self, address: u16, value: u8) {
        let (borrowed, ref mut data) = *self.value.borrow_mut();
        if !borrowed || self.borrower {
            data[address as usize] = value;
        }
    }

    fn read_value(&self, address: u16) -> u8 {
        let (borrowed, ref data) = *self.value.borrow();
        if borrowed && !self.borrower {
            0xFF
        } else {
            data[address as usize]
        }
    }

    fn clone(&mut self) -> MemoryChunk {
        MemoryChunk {
            value: self.value.clone(),
            borrower: false,
        }
    }

    fn len(&self) -> u16 {
        let (_, ref data) = *self.value.borrow_mut();
        (*data).len() as u16
    }

    fn from_range(range: Range<u16>) -> MemoryChunk {
        let mut v = Vec::<u8>::new();
        v.resize(range.len(), 0);
        return MemoryChunk::new(v);
    }

    fn clone_from_slice(&mut self, slice: &[u8]) {
        let (ref borrowed, ref mut data) = *self.value.borrow_mut();
        assert!(!borrowed);
        (*data).clone_from_slice(slice);
    }

    fn clone_range_from_slice(&mut self, range: Range<usize>, slice: &[u8]) {
        let (ref borrowed, ref mut data) = *self.value.borrow_mut();
        assert!(!borrowed);
        ((*data)[range]).clone_from_slice(slice);
    }

    fn ptr_eq(&self, other: &MemoryChunk) -> bool {
        Rc::<RefCell<(bool, Vec<u8>)>>::ptr_eq(&self.value, &other.value)
    }

    fn borrow(&mut self) {
        if self.borrower {
            return;
        }

        let (ref mut borrowed, _) = *self.value.borrow_mut();
        assert!(!*borrowed);
        *borrowed = true;
        self.borrower = true;
    }

    fn release(&mut self) {
        assert!(self.borrower);
        self.borrower = false;

        let (ref mut borrowed, _) = *self.value.borrow_mut();
        assert!(*borrowed);
        *borrowed = false;
    }
}

struct MemoryChunkIterator<'a> {
    chunk: &'a MemoryChunk,
    current: u16,
}

impl<'a> Iterator for MemoryChunkIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
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
    fn new(chunk: &'a MemoryChunk) -> MemoryChunkIterator {
        return MemoryChunkIterator {
            chunk: chunk,
            current: 0,
        };
    }
}

#[test]
#[should_panic]
fn overlapping_inside_chunk() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(12, MemoryChunk::new(vec![1, 2, 3]));
    mm.map_chunk(13, MemoryChunk::new(vec![1]));
}

#[test]
#[should_panic]
fn overlapping_left_side_chunk() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(12, MemoryChunk::new(vec![1, 2, 3]));
    mm.map_chunk(10, MemoryChunk::new(vec![1, 2, 3]));
}

#[test]
#[should_panic]
fn overlapping_right_side_chunk() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(12, MemoryChunk::new(vec![1, 2, 3]));
    mm.map_chunk(14, MemoryChunk::new(vec![1, 2, 3]));
}

#[test]
fn non_overlapping_left_chunk() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(3, MemoryChunk::new(vec![1, 2, 3]));
    mm.map_chunk(0, MemoryChunk::new(vec![1, 2, 3]));
}

#[test]
fn non_overlapping_right_chunk() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(12, MemoryChunk::new(vec![1, 2, 3]));
    mm.map_chunk(15, MemoryChunk::new(vec![1, 2, 3]));
}

#[test]
#[should_panic]
fn mapping_past_end_of_range() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(0xFFFF, MemoryChunk::new(vec![1, 2, 3]));
}

#[test]
fn accessing_unmapped_region() {
    let mm = GameBoyMemoryMap::new();
    assert_eq!(mm.read_memory(24), 0xff);
}

#[test]
fn accessing_unmapped_region_with_region_mapped() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(10, MemoryChunk::new(vec![9, 8, 7]));
    assert_eq!(mm.read_memory(24), 0xff);
}

#[test]
fn setting_unmapped_region() {
    let mut mm = GameBoyMemoryMap::new();
    mm.set_memory(24, 99);
    assert_eq!(mm.read_memory(24), 0xFF);
}

#[test]
fn setting_unmapped_region_with_region_mapped() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(10, MemoryChunk::new(vec![9, 8, 7]));
    mm.set_memory(24, 99);
    assert_eq!(mm.read_memory(24), 0xFF);
}

#[test]
fn accessing_mapped_region() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(10, MemoryChunk::new(vec![9, 8, 7]));
    assert_eq!(mm.read_memory(10), 9);
    assert_eq!(mm.read_memory(11), 8);
    assert_eq!(mm.read_memory(12), 7);
}

#[test]
fn accessing_end_of_address_range() {
    let mut mm = GameBoyMemoryMap::new();
    mm.map_chunk(0xFFFF, MemoryChunk::new(vec![9]));
    assert_eq!(mm.read_memory(0xFFFF), 9);
}

#[test]
fn setting_end_of_address_range() {
    let mut mm = GameBoyMemoryMap::new();
    let mut chunk = MemoryChunk::new(vec![9]);
    mm.map_chunk(0xFFFF, chunk.clone());
    mm.set_memory(0xFFFF, 99);
    assert_eq!(chunk.read_value(0), 99);
}

#[test]
fn setting_mapped_region() {
    let mut mm = GameBoyMemoryMap::new();
    let mut chunk = MemoryChunk::new(vec![9, 8, 7]);
    mm.map_chunk(10, chunk.clone());
    mm.set_memory(11, 88);
    assert_eq!(chunk.read_value(1), 88);
}

#[test]
fn memory_chunk_from_range() {
    let mm = MemoryChunk::from_range(0..10);
    assert_eq!(mm.len(), 10);
}

/*  _     ____ ____   ____            _             _ _
 * | |   / ___|  _ \ / ___|___  _ __ | |_ _ __ ___ | | | ___ _ __
 * | |  | |   | | | | |   / _ \| '_ \| __| '__/ _ \| | |/ _ \ '__|
 * | |__| |___| |_| | |__| (_) | | | | |_| | | (_) | | |  __/ |
 * |_____\____|____/ \____\___/|_| |_|\__|_|  \___/|_|_|\___|_|
 */

const VERTICAL_BLANKING_INTERRUPT_ADDRESS: u16 = 0x0040;
const LCDCSTATUS_INTERRUPT_ADDRESS: u16 = 0x0048;
const TIMER_INTERRUPT_ADDRESS: u16 = 0x0050;

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
const OAM_DATA: Range<u16> = Range {
    start: 0xFE00,
    end: 0xFEA0,
};

/*
 * Number of pixels (both horizontal and vertical) on the screen per gameboy pixel.
 */
const PIXEL_SCALE: u32 = 4;

const CHARACTER_SIZE: u8 = 8;
const CHARACTER_AREA_SIZE: u16 = 32;

#[derive(Default)]
struct Scheduler<T> {
    timeline: BTreeMap<u64, Vec<for<'r> fn(&'r mut T, u64)>>,
}

impl<T> Scheduler<T> {
    fn new() -> Self {
        Scheduler {
            timeline: BTreeMap::new(),
        }
    }

    fn schedule(&mut self, time: u64, event: for<'r> fn(&'r mut T, u64)) {
        if !self.timeline.contains_key(&time) {
            self.timeline.insert(time, Vec::new());
        }
        self.timeline.get_mut(&time).unwrap().push(event);
    }

    fn poll(&mut self, current_time: u64) -> Vec<(u64, for<'r> fn(&'r mut T, u64))> {
        // Find the times that have happened and have events.
        let mut times_to_remove = vec![];
        for &time in self.timeline.keys() {
            if time > current_time {
                break;
            }
            times_to_remove.push(time);
        }

        // Collect all the events to deliver
        let events = times_to_remove
            .iter()
            .map(|v| (*v, self.timeline.remove(v).unwrap()))
            .fold(vec![], |mut s, (t, ref v)| {
                s.append(&mut v.iter().map(|&v| (t, v)).collect());
                return s;
            });

        return events;
    }
}

#[derive(Default)]
struct LCDControllerRegisters {
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
enum LCDControlFlag {
    OperationStop = 0b10000000,
    WindowCodeAreaSelection = 0b01000000,
    WindowingOn = 0b00100000,
    BGCharacterDataSelection = 0b00010000,
    BGCodeAreaSelection = 0b00001000,
    ObjectBlockCompositionSelection = 0b00000100,
    ObjectOn = 0b00000010,
    BGDisplayOn = 0b00000001,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LCDStatusFlag {
    InterruptLYMatching = 0b10000000,
    InterruptMode10 = 0b01000000,
    InterruptMode01 = 0b00100000,
    InterruptMode00 = 0b00010000,
    LYMatch = 0b00001000,
    Unknown = 0b00000100,
    Mode = 0b00000011,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum InterruptFlag {
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

    fn draw(&self, renderer: &mut sdl2::render::Renderer, x: i32, y: i32, ly: u8) {
        for (p, shade) in self.data.iter().enumerate() {
            let (offset_x, offset_y) = ((p as u8 % CHARACTER_SIZE), (p as u8 / CHARACTER_SIZE));
            if y + offset_y as i32 == ly as i32 {
                let rect = sdl2::rect::Rect::new(
                    (x + offset_x as i32) * PIXEL_SCALE as i32,
                    (y + offset_y as i32) * PIXEL_SCALE as i32,
                    PIXEL_SCALE,
                    PIXEL_SCALE,
                );
                let color = color_for_shade(*shade);
                renderer.set_draw_color(color);
                renderer.fill_rect(rect).unwrap();
            }
        }
    }
}

struct LCDController<'a> {
    renderer: Option<sdl2::render::Renderer<'a>>,
    event_pump: Option<sdl2::EventPump>,
    pub crash_message: Option<String>,
    character_data: MemoryChunk,
    background_display_data_1: MemoryChunk,
    background_display_data_2: MemoryChunk,
    oam_data: MemoryChunk,
    unusable_memory: MemoryChunk,
    registers: LCDControllerRegisters,
    scheduler: Scheduler<LCDController<'a>>,
}

impl<'a> LCDController<'a> {
    fn new() -> Self {
        LCDController {
            renderer: None,
            event_pump: None,
            crash_message: None,
            character_data: MemoryChunk::from_range(CHARACTER_DATA),
            background_display_data_1: MemoryChunk::from_range(BACKGROUND_DISPLAY_DATA_1),
            background_display_data_2: MemoryChunk::from_range(BACKGROUND_DISPLAY_DATA_2),
            oam_data: MemoryChunk::from_range(OAM_DATA),
            unusable_memory: MemoryChunk::from_range(UNUSABLE_MEMORY),
            registers: Default::default(),
            scheduler: Scheduler::new(),
        }
    }

    fn start_rendering(&mut self) {
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
        renderer.present();

        self.renderer = Some(renderer);
        self.event_pump = Some(event_pump);
    }

    fn read_dot_data(&self, character_code: u8) -> LCDDotData {
        let mut dot_data = LCDDotData::new();

        let location = if self.read_lcd_control_flag(LCDControlFlag::BGCharacterDataSelection) {
            CHARACTER_DATA_1.start
        } else {
            CHARACTER_DATA_2.start
        } as usize + character_code as usize * 16;

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

    fn set_state_post_bios(&mut self) {
        self.set_lcd_control_flag(LCDControlFlag::OperationStop, true);
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

    fn crashed(&self) -> bool {
        self.crash_message.is_some()
    }

    fn draw(&mut self) {
        if self.renderer.is_none() {
            return;
        }

        for event in self.event_pump.as_mut().unwrap().poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    self.crash_message = Some(String::from("Screen Closed"));
                    return;
                }
                _ => {}
            }
        }

        let (scroll_x, scroll_y) = self.get_scroll_origin_relative_to_lcd();

        let iter = match self.read_lcd_control_flag(LCDControlFlag::BGCodeAreaSelection) {
            false => MemoryChunkIterator::new(&self.background_display_data_1),
            true => MemoryChunkIterator::new(&self.background_display_data_2),
        }.enumerate();

        let ly = self.registers.ly.read_value();

        for (c, character_code) in iter {
            let character_data = self.read_dot_data(character_code);
            let character_x =
                scroll_x + ((c as u16 % CHARACTER_AREA_SIZE) as u8 * CHARACTER_SIZE) as i32;
            let character_y =
                scroll_y + ((c as u16 / CHARACTER_AREA_SIZE) as u8 * CHARACTER_SIZE) as i32;
            character_data.draw(
                self.renderer.as_mut().unwrap(),
                character_x,
                character_y,
                ly,
            );
        }

        let (window_x, window_y) = self.get_window_origin_relative_to_lcd();
        let iter = LCDObjectIterator::new(&self.oam_data);
        for object in iter {
            let character_data = self.read_dot_data(object.character_code);
            character_data.draw(
                self.renderer.as_mut().unwrap(),
                window_x + object.x_coordinate as i32,
                window_y + object.y_coordinate as i32,
                ly,
            );
        }

        if ly == 143 {
            self.renderer.as_mut().unwrap().present();
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

    fn mode_2(&mut self, time: u64) {
        if self.registers.ly.read_value() < 144 {
            self.oam_data.borrow();
            self.unusable_memory.borrow();
            self.set_lcd_status_mode(0x2);
            self.draw();
        }
        self.scheduler.schedule(time + 77, Self::mode_3);
    }

    fn mode_3(&mut self, time: u64) {
        if self.registers.ly.read_value() < 144 {
            self.character_data.borrow();
            self.background_display_data_1.borrow();
            self.background_display_data_2.borrow();
            self.set_lcd_status_mode(0x3);
        }
        self.scheduler.schedule(time + 175, Self::mode_0);
    }

    fn mode_0(&mut self, time: u64) {
        if self.registers.ly.read_value() < 144 {
            self.character_data.release();
            self.background_display_data_1.release();
            self.background_display_data_2.release();
            self.oam_data.release();
            self.unusable_memory.release();

            self.set_lcd_status_mode(0x0);

            // let interrupt_flag_value = interrupt_flag.read_value();
            // interrupt_flag.set_value(interrupt_flag_value | InterruptFlag::VerticalBlanking as u8);

            // let ly = self.ly.read_value();
            // let lyc = self.lyc.read_value();
            // interrupt_flag.set_value(interrupt_flag_value | InterruptFlag::LCDSTAT as u8);
            // self.set_lcd_status_flag(LCDStatusFlag::InterruptLYMatching, true);

            // self.lcd_controller
            //     .set_lcd_status_flag(LCDStatusFlag::LYMatch, ly == lyc);
            self.scheduler.schedule(time + 204, Self::mode_2);
        } else {
            self.scheduler.schedule(time + 204, Self::mode_1);
        }
    }

    fn advance_ly(&mut self, time: u64) {
        // This advances the ly register, which represent the horizontal line that is currently
        // being drawn on the LCD.

        if self.registers.ly.read_value() >= 153 {
            self.registers.ly.set_value(0);
        } else {
            self.registers.ly.add(1);
        }

        self.scheduler.schedule(time + 456, Self::advance_ly);
    }

    fn unknown_event(&mut self, time: u64) {
        self.set_lcd_status_flag(LCDStatusFlag::Unknown, false);

        if self.registers.ly.read_value() < 144 {
            self.oam_data.borrow();
            self.unusable_memory.borrow();
        }
        self.scheduler.schedule(time + 456, Self::unknown_event);
    }

    fn mode_1(&mut self, time: u64) {
        self.set_lcd_status_mode(0x1);

        self.scheduler.schedule(time + 4560, Self::mode_2);
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
const UNUSABLE_MEMORY: Range<u16> = Range {
    start: 0xFEA0,
    end: 0xFF00,
};
const IO_PORTS_A: Range<u16> = Range {
    start: 0xFF10,
    end: 0xFF40,
};
const IO_PORTS_B: Range<u16> = Range {
    start: 0xFF4C,
    end: 0xFF80,
};
const HIGH_RAM: Range<u16> = Range {
    start: 0xFF80,
    end: 0xFFFF,
};
const INTERNAL_RAM_A: Range<u16> = Range {
    start: 0xC000,
    end: 0xDE00,
};
const INTERNAL_RAM_B: Range<u16> = Range {
    start: 0xDE00,
    end: 0xE000,
};
const ECHO_RAM: Range<u16> = Range {
    start: 0xE000,
    end: 0xFE00,
};

struct GameBoyRegister {
    chunk: MemoryChunk,
}

impl Default for GameBoyRegister {
    fn default() -> Self {
        GameBoyRegister::new()
    }
}

impl GameBoyRegister {
    fn new() -> GameBoyRegister {
        GameBoyRegister {
            chunk: MemoryChunk::from_range(0..1),
        }
    }

    fn read_value(&self) -> u8 {
        self.chunk.read_value(0)
    }

    fn set_value(&mut self, value: u8) {
        self.chunk.set_value(0, value)
    }

    fn add(&mut self, value: u8) {
        let old_value = self.read_value();
        self.set_value(old_value.wrapping_add(value));
    }

    // XXX this isn't used, but might be useful.
    #[allow(dead_code)]
    fn subtract(&mut self, value: u8) {
        let old_value = self.read_value();
        self.set_value(old_value.wrapping_sub(value));
    }
}

#[derive(Default)]
struct GameBoyRegisters {
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

struct GameBoyEmulator<'a> {
    cpu: LR35902Emulator<GameBoyMemoryMap>,
    lcd_controller: LCDController<'a>,
    io_ports_a: MemoryChunk,
    io_ports_b: MemoryChunk,
    high_ram: MemoryChunk,
    internal_ram_a: MemoryChunk,
    internal_ram_b: MemoryChunk,

    registers: GameBoyRegisters,
    scheduler: Scheduler<GameBoyEmulator<'a>>,
}

impl<'a> GameBoyEmulator<'a> {
    fn new() -> GameBoyEmulator<'a> {
        let mut e = GameBoyEmulator {
            cpu: LR35902Emulator::new(GameBoyMemoryMap::new()),
            lcd_controller: LCDController::new(),
            io_ports_a: MemoryChunk::from_range(IO_PORTS_A),
            io_ports_b: MemoryChunk::from_range(IO_PORTS_B),
            high_ram: MemoryChunk::from_range(HIGH_RAM),
            internal_ram_a: MemoryChunk::from_range(INTERNAL_RAM_A),
            internal_ram_b: MemoryChunk::from_range(INTERNAL_RAM_B),
            registers: Default::default(),
            scheduler: Scheduler::new(),
        };

        // Restart and interrupt vectors (unmapped) 0x0000 - 0x00FF

        // Rom (unmapped) 0x0100 - 0x7FFF

        // Character data 0x8000 - 0x97FF
        e.cpu.memory_accessor.map_chunk(
            CHARACTER_DATA.start,
            e.lcd_controller.character_data.clone(),
        );

        // Background display data 0x9800 - 0x9FFF
        e.cpu.memory_accessor.map_chunk(
            BACKGROUND_DISPLAY_DATA_1.start,
            e.lcd_controller.background_display_data_1.clone(),
        );
        e.cpu.memory_accessor.map_chunk(
            BACKGROUND_DISPLAY_DATA_2.start,
            e.lcd_controller.background_display_data_2.clone(),
        );

        // Cartridge RAM (unmapped) 0xA000 - 0xBFFF

        // Internal RAM 0xC000 - 0xDFFF
        e.cpu
            .memory_accessor
            .map_chunk(INTERNAL_RAM_A.start, e.internal_ram_a.clone());
        e.cpu
            .memory_accessor
            .map_chunk(INTERNAL_RAM_B.start, e.internal_ram_b.clone());

        // Echo RAM 0xE000 - 0xFDFF
        e.cpu
            .memory_accessor
            .map_chunk(ECHO_RAM.start, e.internal_ram_a.clone());

        // OAM Data 0xFE00 - 0xFE9F
        e.cpu
            .memory_accessor
            .map_chunk(OAM_DATA.start, e.lcd_controller.oam_data.clone());

        // Unusable memory 0xFEA0 - 0xFEFF
        e.cpu.memory_accessor.map_chunk(
            UNUSABLE_MEMORY.start,
            e.lcd_controller.unusable_memory.clone(),
        );

        // Registers
        e.cpu
            .memory_accessor
            .map_chunk(0xFF00, e.registers.p1_joypad.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF01, e.registers.serial_transfer_data.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF02, e.registers.serial_transfer_control.chunk.clone());

        e.cpu
            .memory_accessor
            .map_chunk(0xFF04, e.registers.divider.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF05, e.registers.timer_counter.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF06, e.registers.timer_modulo.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF07, e.registers.timer_control.chunk.clone());

        e.cpu
            .memory_accessor
            .map_chunk(0xFF0F, e.registers.interrupt_flag.chunk.clone());

        // Other IO Registers 0xFF10 - 0xFF3F
        e.cpu
            .memory_accessor
            .map_chunk(IO_PORTS_A.start, e.io_ports_a.clone());

        // LCD Registers 0xFF40 - 0xFF4B
        e.cpu
            .memory_accessor
            .map_chunk(0xFF40, e.lcd_controller.registers.lcdc.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF41, e.lcd_controller.registers.stat.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF42, e.lcd_controller.registers.scy.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF43, e.lcd_controller.registers.scx.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF44, e.lcd_controller.registers.ly.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF45, e.lcd_controller.registers.lyc.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF46, e.lcd_controller.registers.dma.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF47, e.lcd_controller.registers.bgp.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF48, e.lcd_controller.registers.obp0.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF49, e.lcd_controller.registers.obp1.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF4A, e.lcd_controller.registers.wy.chunk.clone());
        e.cpu
            .memory_accessor
            .map_chunk(0xFF4B, e.lcd_controller.registers.wx.chunk.clone());

        // Other IO Registers 0xFF4C - 0xFF7F
        e.cpu
            .memory_accessor
            .map_chunk(IO_PORTS_B.start, e.io_ports_b.clone());

        // High RAM 0xFF80 - 0xFFFE
        e.cpu
            .memory_accessor
            .map_chunk(HIGH_RAM.start, e.high_ram.clone());

        // interrupt enable register
        e.cpu
            .memory_accessor
            .map_chunk(0xFFFF, e.registers.interrupt_enable.chunk.clone());

        e.lcd_controller.set_state_post_bios();

        e.set_state_post_bios();

        let elapsed_cycles = e.cpu.elapsed_cycles;
        e.scheduler
            .schedule(elapsed_cycles + 52, Self::divider_tick);

        e.lcd_controller
            .scheduler
            .schedule(elapsed_cycles + 56 + 4, LCDController::mode_2);
        e.lcd_controller
            .scheduler
            .schedule(elapsed_cycles + 56 + 456, LCDController::advance_ly);

        e.lcd_controller
            .scheduler
            .schedule(102860, LCDController::unknown_event);

        return e;
    }

    fn load_rom(&mut self, rom: &[u8]) {
        self.cpu
            .memory_accessor
            .map_chunk(0, MemoryChunk::new(rom.to_vec()));
    }

    fn crashed(&self) -> Option<&String> {
        if self.cpu.crashed() {
            return self.cpu.crash_message.as_ref();
        } else if self.lcd_controller.crashed() {
            return self.lcd_controller.crash_message.as_ref();
        }

        None
    }

    fn divider_tick(&mut self, time: u64) {
        self.registers.divider.add(1);
        self.scheduler.schedule(time + 256, Self::divider_tick);
    }

    fn tick(&mut self) {
        self.cpu.run_one_instruction();

        let current_clock = self.cpu.elapsed_cycles;

        for (time, event) in self.scheduler.poll(current_clock) {
            event(self, time);
        }

        for (time, event) in self.lcd_controller.scheduler.poll(current_clock) {
            event(&mut self.lcd_controller, time);
        }

        if self.cpu.get_interrupts_enabled() {
            self.handle_interrupts();
        }
    }

    fn handle_interrupts(&mut self) {
        let interrupt_flag_value = self.registers.interrupt_flag.read_value();
        let interrupt_enable_value = self.registers.interrupt_enable.read_value();

        if interrupt_flag_value & InterruptFlag::VerticalBlanking as u8 != 0
            && interrupt_enable_value & InterruptFlag::VerticalBlanking as u8 != 0
        {
            self.registers
                .interrupt_flag
                .set_value(interrupt_flag_value & !(InterruptFlag::VerticalBlanking as u8));
            self.cpu.interrupt(VERTICAL_BLANKING_INTERRUPT_ADDRESS);
        }

        if interrupt_flag_value & InterruptFlag::LCDSTAT as u8 != 0
            && interrupt_enable_value & InterruptFlag::LCDSTAT as u8 != 0
        {
            self.cpu.interrupt(LCDCSTATUS_INTERRUPT_ADDRESS);
        }

        if interrupt_flag_value & InterruptFlag::Timer as u8 != 0
            && interrupt_enable_value & InterruptFlag::Timer as u8 != 0
        {
            self.cpu.interrupt(TIMER_INTERRUPT_ADDRESS);
        }
    }

    fn set_state_post_bios(&mut self) {
        /*
         * After running the BIOS (the part of the gameboy that shows the logo) the cpu is left in
         * a very certain state. Since this is always the case, certain games may rely on this fact
         * (and indeed often times do.)
         */
        self.cpu.set_register(Intel8080Register::A, 0x1);
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

        self.registers.p1_joypad.set_value(0xcf);
        self.registers.serial_transfer_data.set_value(0x0);
        self.registers.serial_transfer_control.set_value(0x7e);

        self.registers.divider.set_value(0xab);
        self.registers.timer_counter.set_value(0x0);
        self.registers.timer_modulo.set_value(0x0);
        self.registers.timer_control.set_value(0xf8);

        self.registers.interrupt_flag.set_value(0xe1);

        // Initialize io ports
        let io_ports_a = include_bytes!("assets/io_ports_a.bin");
        self.io_ports_a.clone_from_slice(&io_ports_a[..]);

        /* 40 - 4B LCD Controller */

        let io_ports_b = include_bytes!("assets/io_ports_b.bin");
        self.io_ports_b.clone_from_slice(&io_ports_b[..]);

        let high_ram = include_bytes!("assets/high_ram.bin");
        self.high_ram.clone_from_slice(&high_ram[..]);

        let internal_ram = include_bytes!("assets/internal_ram.bin");

        let split = (INTERNAL_RAM_A.end - INTERNAL_RAM_A.start) as usize;
        self.internal_ram_a
            .clone_from_slice(&internal_ram[0..split]);
        self.internal_ram_b.clone_from_slice(&internal_ram[split..]);

        self.registers.interrupt_enable.set_value(0x0);
    }

    fn run(&mut self) {
        self.lcd_controller.start_rendering();
        while self.crashed().is_none() {
            self.tick();
        }
        if self.cpu.crashed() {
            println!(
                "Emulator crashed: {}",
                self.cpu.crash_message.as_ref().unwrap()
            );
        }
    }

    fn write_memory(&self, w: &mut Write) -> io::Result<()> {
        let mut mem = [0u8; 0x10000];
        for i in 0..0x10000 {
            mem[i] = self.cpu.memory_accessor.read_memory(i as u16);
        }

        w.write(&mem)?;
        Ok(())
    }

    fn hash(&self) -> u32 {
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
        mem[0x10000 + 11] = if self.cpu.read_flag(LR35902Flag::Zero) {
            1
        } else {
            0
        };
        mem[0x10000 + 12] = if self.cpu.read_flag(LR35902Flag::Subtract) {
            1
        } else {
            0
        };
        mem[0x10000 + 13] = if self.cpu.read_flag(LR35902Flag::HalfCarry) {
            1
        } else {
            0
        };
        mem[0x10000 + 14] = if self.cpu.read_flag(LR35902Flag::Carry) {
            1
        } else {
            0
        };
        return super_fast_hash(&mem);
    }
}

#[test]
fn initial_state_test() {
    let e = GameBoyEmulator::new();

    // Lock down the initial state.
    assert_eq!(e.hash(), 1497694477);
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
fn blargg_test_rom_cpu_instrs_2_interrupts() {
    let mut e = GameBoyEmulator::new();
    e.load_rom(&read_blargg_test_rom(
        "cpu_instrs/individual/02-interrupts.gb",
    ));
    run_blargg_test_rom(&mut e.cpu, 0xc7f4);
}

pub fn run_emulator(rom: &[u8]) {
    let mut e = GameBoyEmulator::new();
    e.load_rom(&rom);
    e.run();
}

pub fn run_in_tandem_with(other_emulator_path: &str, rom: &[u8]) {
    println!("loading {}", other_emulator_path);

    tandem::run(other_emulator_path, rom);
}
