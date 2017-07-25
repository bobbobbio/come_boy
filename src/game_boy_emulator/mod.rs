// Copyright 2017 Remi Bernotavicius

extern crate sdl2;

mod debugger;
mod disassembler;

use std::ops::Range;
use std::{iter, time};

pub use game_boy_emulator::debugger::run_debugger;
use lr35902_emulator::{LR35902Emulator, LR35902Flag, Intel8080Register};
use util::super_fast_hash;
use emulator_common::{MemoryAccessor, MemoryIterator};

pub use game_boy_emulator::disassembler::disassemble_game_boy_rom;

const IF_REGISTER: u16 = 0xFF0F;
const IE_REGISTER: u16 = 0xFFFF;

struct GameBoyMemoryMap {
    pub memory: [u8; 0x10000],
}

impl MemoryAccessor for GameBoyMemoryMap {
    fn read_memory(&self, address: u16) -> u8
    {
        return self.memory[address as usize];
    }

    fn set_memory(&mut self, address: u16, value: u8)
    {
        self.memory[address as usize] = value;
    }
}

impl GameBoyMemoryMap {
    fn new() -> GameBoyMemoryMap
    {
        return GameBoyMemoryMap {
            memory: [0; 0x10000]
        };
    }
}

/*  _     ____ ____   ____            _             _ _
 * | |   / ___|  _ \ / ___|___  _ __ | |_ _ __ ___ | | | ___ _ __
 * | |  | |   | | | | |   / _ \| '_ \| __| '__/ _ \| | |/ _ \ '__|
 * | |__| |___| |_| | |__| (_) | | | | |_| | | (_) | | |  __/ |
 * |_____\____|____/ \____\___/|_| |_|\__|_|  \___/|_|_|\___|_|
 */

struct LCDController<'a> {
    renderer: sdl2::render::Renderer<'a>,
    event_pump: sdl2::EventPump,
    pub crash_message: Option<String>
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
enum LCDRegister {
    LCDC = 0xFF40,
    STAT = 0xFF41,
    SCY  = 0xFF42,
    SCX  = 0xFF43,
    LY   = 0xFF44,
    LYC  = 0xFF45,
    // DMA  = 0xFF46,
    BGP  = 0xFF47,
    OBP0 = 0xFF48,
    OBP1 = 0xFF49,
    WY   = 0xFF4A,
    WX   = 0xFF4B,
}

#[derive(Debug,Clone,Copy,PartialEq)]
enum LCDControlFlag {
                                     // 76543210
    OperationStop =                   0b10000000,
    // WindowCodeAreaSelection =         0b01000000,
    // WindowingOn =                     0b00100000,
    BGCharacterDataSelection =        0b00010000,
    BGCodeAreaSelection =             0b00001000,
    // ObjectBlockCompositionSelection = 0b00000100,
    // ObjectOn =                        0b00000010,
    BGDisplayOn =                     0b00000001,
}

#[derive(Debug,Clone,Copy,PartialEq)]
enum LCDStatusFlag {
                         // 76543210
    InterruptLYMatching = 0b10000000,
    // InterruptMode10 =     0b01000000,
    // InterruptMode01 =     0b00100000,
    // InterruptMode00 =     0b00010000,
    LYMatch =             0b00001000,
    Mode =                0b00000011,
}

#[derive(Debug,Clone,Copy,PartialEq)]
enum LCDInterruptFlag {
                      // 76543210
    LCDC =             0b00001000,
    VerticalBlanking = 0b00000011,
}

const VERTICAL_BLANKING_INTERRUPT_ADDRESS : u16 = 0x0040;
const LCDCSTATUS_INTERRUPT_ADDRESS : u16 = 0x0048;

const CHARACTER_DATA_ADDRESS_1: Range<u16> = Range { start: 0x8000, end: 0x9000};
const CHARACTER_DATA_ADDRESS_2: Range<u16> = Range { start: 0x8800, end: 0x9800};
const BACKGROUND_DISPLAY_DATA_1: Range<u16> = Range { start: 0x9800, end: 0x9C00 };
const BACKGROUND_DISPLAY_DATA_2: Range<u16> = Range { start: 0x9C00, end: 0xA000 };
const OAM_DATA: Range<u16> = Range { start: 0xFE00, end: 0xFEA0 };

/*
 * Number of pixels (both horizontal and vertical) on the screen per gameboy pixel.
 */
const PIXEL_SCALE: u32 = 4;

const CHARACTER_SIZE: u8 = 8;
const CHARACTER_AREA_SIZE: u16 = 32;

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
    memory_iterator: iter::Peekable<MemoryIterator<'a>>
}

impl<'a> Iterator for LCDObjectIterator<'a> {
    type Item = LCDObject;

    fn next(&mut self) -> Option<LCDObject>
    {
        if self.memory_iterator.peek() == None {
            return None
        } else {
            let lcd_object = LCDObject {
                y_coordinate: self.memory_iterator.next().unwrap(),
                x_coordinate: self.memory_iterator.next().unwrap(),
                character_code: self.memory_iterator.next().unwrap(),
                flags: self.memory_iterator.next().unwrap()
            };
            return Some(lcd_object);
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
                ((x + offset_x as i32) * PIXEL_SCALE as i32),
                ((y + offset_y as i32) * PIXEL_SCALE as i32), PIXEL_SCALE, PIXEL_SCALE);
            let color = color_for_shade(*shade);
            renderer.set_draw_color(color);
            renderer.fill_rect(rect).unwrap();
        }
    }
}

impl<'a> LCDController<'a> {
    fn new() -> LCDController<'a>
    {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("come boy", 160 * PIXEL_SCALE, 144 * PIXEL_SCALE)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut renderer = window.renderer().build().unwrap();

        renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        renderer.clear();
        renderer.present();

        let event_pump = sdl_context.event_pump().unwrap();

        LCDController {
            renderer: renderer,
            event_pump: event_pump,
            crash_message: None
        }
    }

    fn iterate_background_display_data<'b>(
        &self, cpu: &'b LR35902Emulator<GameBoyMemoryMap>) -> MemoryIterator<'b>
    {
        match self.read_lcd_control_flag(cpu, LCDControlFlag::BGCodeAreaSelection) {
            false => MemoryIterator::new(&cpu.memory_accessor, BACKGROUND_DISPLAY_DATA_1),
            true => MemoryIterator::new(&cpu.memory_accessor, BACKGROUND_DISPLAY_DATA_2),
        }
    }

    fn read_dot_data(&self, cpu: &LR35902Emulator<GameBoyMemoryMap>, character_code: u8) -> LCDDotData
    {
        let mut dot_data = LCDDotData::new();
        let cd_addr = if self.read_lcd_control_flag(
            cpu, LCDControlFlag::BGCharacterDataSelection) {
            CHARACTER_DATA_ADDRESS_1.start
        } else {
            CHARACTER_DATA_ADDRESS_2.start
        };
        let location = character_code as u16 * 16 + cd_addr;
        let mut i = 0;
        let mut iter = MemoryIterator::new(
            &cpu.memory_accessor, location..location + 16).peekable();
        while iter.peek() != None {
            let byte1 = iter.next().unwrap();
            let byte2 = iter.next().unwrap();
            for bit in (0..8).rev() {
                let shade_upper = byte1 >> (bit - 1) & 0x2;
                let shade_lower = byte2 >> bit & 0x1;
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

    fn iterate_objects<'b>(&self, cpu: &'b LR35902Emulator<GameBoyMemoryMap>) -> LCDObjectIterator<'b>
    {
        LCDObjectIterator {
            memory_iterator: MemoryIterator::new(&cpu.memory_accessor, OAM_DATA).peekable()
        }
    }

    fn read_register(&self, cpu: &LR35902Emulator<GameBoyMemoryMap>, register: LCDRegister) -> u8
    {
        cpu.read_memory(register as u16)
    }

    fn set_register(&self, cpu: &mut LR35902Emulator<GameBoyMemoryMap>, register: LCDRegister, value: u8)
    {
        cpu.set_memory(register as u16, value);
    }

    fn read_lcd_control_flag(&self, cpu: &LR35902Emulator<GameBoyMemoryMap>, flag: LCDControlFlag) -> bool
    {
        let lcdc = self.read_register(cpu, LCDRegister::LCDC);
        return lcdc & flag as u8 == flag as u8;
    }

    fn set_lcd_control_flag(&self, cpu: &mut LR35902Emulator<GameBoyMemoryMap>, flag: LCDControlFlag, value: bool)
    {
        let lcdc = self.read_register(cpu, LCDRegister::LCDC);
        if value {
            self.set_register(cpu, LCDRegister::LCDC, lcdc | flag as u8);
        } else {
            self.set_register(cpu, LCDRegister::LCDC, lcdc & !(flag as u8));
        }
    }

    /*
    fn read_lcd_status_flag(&self, cpu: &LR35902Emulator<GameBoyMemoryMap>, flag: LCDStatusFlag) -> bool
    {
        let stat = self.read_register(cpu, LCDRegister::STAT);
        return stat & flag as u8 == flag as u8;
    }
    */

    fn set_lcd_status_flag(&self, cpu: &mut LR35902Emulator<GameBoyMemoryMap>, flag: LCDStatusFlag, value: bool)
    {
        // Mode is a four-value flag
        assert!(flag != LCDStatusFlag::Mode);

        let stat = self.read_register(cpu, LCDRegister::STAT);
        if value {
            self.set_register(cpu, LCDRegister::STAT, stat | flag as u8);
        } else {
            self.set_register(cpu, LCDRegister::STAT, stat & !(flag as u8));
        }
    }

    fn set_lcd_status_mode(&self, cpu: &mut LR35902Emulator<GameBoyMemoryMap>, value: u8)
    {
        let stat = self.read_register(cpu, LCDRegister::STAT) & !(LCDStatusFlag::Mode as u8);
        self.set_register(cpu, LCDRegister::STAT, stat | value);
    }

    fn initialize_flags(&self, cpu: &mut LR35902Emulator<GameBoyMemoryMap>)
    {
        self.set_lcd_control_flag(cpu, LCDControlFlag::OperationStop, true);
        self.set_lcd_control_flag(cpu, LCDControlFlag::BGCharacterDataSelection, true);
        self.set_lcd_control_flag(cpu, LCDControlFlag::BGDisplayOn, true);
        self.set_register(cpu, LCDRegister::BGP, 0xFC);
        self.set_register(cpu, LCDRegister::OBP0, 0xFF);
        self.set_register(cpu, LCDRegister::OBP1, 0xFF);

        self.set_lcd_status_flag(cpu, LCDStatusFlag::InterruptLYMatching, true);
        self.set_lcd_status_mode(cpu, 0x1);
    }

    fn get_scroll_origin_relative_to_lcd(&self, cpu: &LR35902Emulator<GameBoyMemoryMap>) -> (i32, i32)
    {
        let mut x = self.read_register(cpu, LCDRegister::SCX) as i32 * -1;
        let mut y = self.read_register(cpu, LCDRegister::SCY) as i32 * -1;

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

    fn get_window_origin_relative_to_lcd(&self, cpu: &LR35902Emulator<GameBoyMemoryMap>) -> (i32, i32)
    {
        let x = self.read_register(cpu, LCDRegister::WX) as i32 * -1;
        let y = self.read_register(cpu, LCDRegister::WY) as i32 * -1;

        return (x, y);
    }

    fn crashed(&self) -> bool
    {
        self.crash_message.is_some()
    }

    fn draw_one_line(&mut self, cpu: &mut LR35902Emulator<GameBoyMemoryMap>)
    {
        /*
         * Update the LY register which represents the line being draw currently.
         */
        let ly = self.read_register(cpu, LCDRegister::LY);
        if ly == 153 {
            self.set_register(cpu, LCDRegister::LY, 0);
        } else {
            self.set_register(cpu, LCDRegister::LY, ly + 1);
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

        /*
         * Other than checking to see if the user has closed the window, for some reason if we do
         * not poll the event_pump the screen will not draw properly.
         */
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    self.crash_message = Some(String::from("Screen Closed"));
                    return;
                },
                _ => {}
            }
        }

        self.renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        self.renderer.clear();

        let (scroll_x, scroll_y) = self.get_scroll_origin_relative_to_lcd(cpu);

        for (c, character_code) in self.iterate_background_display_data(cpu).enumerate() {
            let character_data = self.read_dot_data(cpu, character_code);
            let character_x = scroll_x
                + ((c as u16 % CHARACTER_AREA_SIZE) as u8 * CHARACTER_SIZE) as i32;
            let character_y = scroll_y
                + ((c as u16 / CHARACTER_AREA_SIZE) as u8 * CHARACTER_SIZE) as i32;
            character_data.draw(&mut self.renderer, character_x, character_y);
        }

        let (window_x, window_y) = self.get_window_origin_relative_to_lcd(cpu);
        for object in self.iterate_objects(cpu) {
            let character_data = self.read_dot_data(cpu, object.character_code);
            character_data.draw(
                &mut self.renderer,
                window_x + object.x_coordinate as i32,
                window_y + object.y_coordinate as i32);
        }

        self.renderer.present();
    }

    fn process_interrupts(&mut self, cpu: &mut LR35902Emulator<GameBoyMemoryMap>)
    {
        let ly = self.read_register(cpu, LCDRegister::LY);

        // XXX This needs to live elsewhere
        let if_register = cpu.read_memory(IF_REGISTER);
        let ie_register = cpu.read_memory(IE_REGISTER);

        // Vertical blanking starts when ly == 144
        if ly == 144 {
            cpu.set_memory(0xFF0F, if_register & LCDInterruptFlag::VerticalBlanking as u8);
            if ie_register & LCDInterruptFlag::VerticalBlanking as u8 != 0 {
                cpu.interrupt(VERTICAL_BLANKING_INTERRUPT_ADDRESS);
            }
        }

        if ly == self.read_register(cpu, LCDRegister::LYC) {
            cpu.set_memory(0xFF0F, if_register & LCDInterruptFlag::LCDC as u8);
            self.set_lcd_status_flag(cpu, LCDStatusFlag::InterruptLYMatching, true);
            self.set_lcd_status_flag(cpu, LCDStatusFlag::LYMatch, true);
            cpu.interrupt(LCDCSTATUS_INTERRUPT_ADDRESS);
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

const CARTRIDGE_RAM: Range<u16> = Range { start: 0xA000, end: 0xC000 };
const UNUSABLE_MEMORY: Range<u16> = Range { start: 0xFEA0, end: 0xFF00 };
const IO_PORTS: Range<u16> = Range { start: 0xFF00, end: 0xFF80 };
const HIGH_RAM: Range<u16> = Range { start: 0xFF80, end: 0xFFFF };

struct GameBoyEmulator<'a> {
    cpu: LR35902Emulator<GameBoyMemoryMap>,
    lcd_controller: LCDController<'a>,
    last_draw: time::SystemTime
}

impl<'a> GameBoyEmulator<'a> {
    fn new() -> GameBoyEmulator<'a> {
        let mut e = GameBoyEmulator {
            cpu: LR35902Emulator::new(GameBoyMemoryMap::new()),
            lcd_controller: LCDController::new(),
            last_draw: time::SystemTime::now()
        };

        e.lcd_controller.initialize_flags(&mut e.cpu);

        e.set_state_post_bios();

        // This is here to prevent the initial state from changing inadvertently.
        assert_eq!(super_fast_hash(&e.cpu.memory_accessor.memory), 2422240235);

        return e;
    }

    fn set_io_port(&mut self, port: usize, value: u8)
    {
        let addr = IO_PORTS.start | (port as u16 & 0x00FF);
        assert!(addr < IO_PORTS.end, "{:02x} >= {:02x}", addr, IO_PORTS.end);
        self.cpu.set_memory(addr, value);
    }

    fn load_rom(&mut self, rom: &[u8])
    {
        self.cpu.memory_accessor.memory[0..rom.len()].clone_from_slice(rom);
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

        // It takes the DMG LCD roughly 10ms to draw one horizontal line.
        if time::SystemTime::now().duration_since(self.last_draw).unwrap() >=
            time::Duration::new(0, 100000) {
            self.lcd_controller.draw_one_line(&mut self.cpu);
            self.lcd_controller.process_interrupts(&mut self.cpu);
            self.last_draw = time::SystemTime::now();
        }
    }

    fn set_state_post_bios(&mut self)
    {
        /*
         * After running the BIOS (the part of the gameboy that shows the logo) the cpu is left in
         * a very certain state. Since this is always the case, certain games may rely on this fact
         * (and indeed often times do.)
         */
        self.cpu.set_register(Intel8080Register::A, 0x01);
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

        // Initialize the cartridge RAM to FF.
        // XXX: why?
        for addr in CARTRIDGE_RAM {
            self.cpu.set_memory(addr, 0xFF);
        }

        // Initialize unusable memory to FF.
        for addr in UNUSABLE_MEMORY {
            self.cpu.set_memory(addr, 0xFF);
        }

        // Initialize io ports
        let io_ports_a = [
            /* 00 - 07 */ 0xcfu8, 0x00u8, 0x7eu8, 0xffu8, 0x69u8, 0x00u8, 0x00u8, 0xf8u8,
            /* 08 - 0f */ 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xe1u8,
            /* 10 - 17 */ 0x80u8, 0xbfu8, 0xf3u8, 0xffu8, 0xbfu8, 0xffu8, 0x3fu8, 0x00u8,
            /* 18 - 1f */ 0xffu8, 0xbfu8, 0x7fu8, 0xffu8, 0x9fu8, 0xffu8, 0xbfu8, 0xffu8,
            /* 20 - 27 */ 0xffu8, 0x00u8, 0x00u8, 0xbfu8, 0x77u8, 0xf3u8, 0xf1u8, 0xffu8,
            /* 28 - 2f */ 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8,
            /* 30 - 37 */ 0x00u8, 0xffu8, 0x00u8, 0xffu8, 0x00u8, 0xffu8, 0x00u8, 0xffu8,
            /* 38 - 3f */ 0x00u8, 0xffu8, 0x00u8, 0xffu8, 0x00u8, 0xffu8, 0x00u8, 0xffu8
        ];

        /* 40 - 4B LCD Controller */

        let io_ports_b = [
            /* 4c - 4f */ 0xffu8, 0x7eu8, 0xffu8, 0x00u8,
            /* 50 - 57 */ 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0x3eu8, 0xffu8,
            /* 58 - 5f */ 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8,
            /* 60 - 67 */ 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8,
            /* 68 - 6f */ 0xc0u8, 0xffu8, 0xc1u8, 0x00u8, 0xffu8, 0xffu8, 0xffu8, 0xffu8,
            /* 70 - 77 */ 0xffu8, 0xffu8, 0x00u8, 0x00u8, 0xffu8, 0x8fu8, 0x00u8, 0x00u8,
            /* 78 - 7f */ 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8
        ];

        for (i, value) in io_ports_a.iter().enumerate() {
            self.set_io_port(i, *value);
        }

        for (i, value) in io_ports_b.iter().enumerate() {
            self.set_io_port(i + 0x4c, *value);
        }

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

        // Initialize high ram.
        for (i, value) in high_ram.iter().enumerate() {
            let addr = HIGH_RAM.start + i as u16;
            assert!(addr < HIGH_RAM.end);
            self.cpu.set_memory(addr, *value);
        }

        self.cpu.set_memory(IE_REGISTER, 0x0);
    }

    fn run(&mut self)
    {
        self.last_draw = time::SystemTime::now();
        while self.crashed().is_none() {
            self.tick()
        }
        if self.cpu.crashed() {
            println!("Emulator crashed: {}", self.cpu.crash_message.as_ref().unwrap());
        }
    }
}

pub fn run_emulator(rom: &[u8])
{
    let mut e = GameBoyEmulator::new();
    e.load_rom(&rom);
    e.run();
}
