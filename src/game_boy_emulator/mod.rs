// Copyright 2017 Remi Bernotavicius

extern crate sdl2;

use std::ops::Range;
use std::{iter, time};

use lr35902_emulator::{LR35902Emulator, LR35902MemoryIterator};

/*  _     ____ ____   ____            _             _ _
 * | |   / ___|  _ \ / ___|___  _ __ | |_ _ __ ___ | | | ___ _ __
 * | |  | |   | | | | |   / _ \| '_ \| __| '__/ _ \| | |/ _ \ '__|
 * | |__| |___| |_| | |__| (_) | | | | |_| | | (_) | | |  __/ |
 * |_____\____|____/ \____\___/|_| |_|\__|_|  \___/|_|_|\___|_|
 */

struct LCDController<'a> {
    renderer: sdl2::render::Renderer<'a>,
    event_pump: sdl2::EventPump,
    crashed: bool,
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
    // BGP  = 0xFF47,
    // OBP0 = 0xFF48,
    // OBP1 = 0xFF49,
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

const CHARACTER_DATA_ADDRESS_1: Range<u16> = Range { start: 0x8800, end: 0x9800};
const CHARACTER_DATA_ADDRESS_2: Range<u16> = Range { start: 0x8000, end: 0x9000};
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
    memory_iterator: iter::Peekable<LR35902MemoryIterator<'a>>
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
            crashed: false,
        }
    }

    fn iterate_background_display_data<'b>(
        &self, cpu: &'b LR35902Emulator) -> LR35902MemoryIterator<'b>
    {
        match self.read_lcd_control_flag(cpu, LCDControlFlag::BGCodeAreaSelection) {
            false => cpu.iterate_memory(BACKGROUND_DISPLAY_DATA_1),
            true => cpu.iterate_memory(BACKGROUND_DISPLAY_DATA_2),
        }
    }

    fn read_dot_data(&self, cpu: &LR35902Emulator, character_code: u8) -> LCDDotData
    {
        let mut dot_data = LCDDotData::new();
        let cd_addr = if !self.read_lcd_control_flag(
            cpu, LCDControlFlag::BGCharacterDataSelection) {
            CHARACTER_DATA_ADDRESS_1.start
        } else {
            CHARACTER_DATA_ADDRESS_2.start
        };
        let location = character_code as u16 * 16 + cd_addr;
        let mut i = 0;
        let mut iter = cpu.iterate_memory(location..location + 16).peekable();
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

    fn iterate_objects<'b>(&self, cpu: &'b LR35902Emulator) -> LCDObjectIterator<'b>
    {
        LCDObjectIterator {
            memory_iterator: cpu.iterate_memory(OAM_DATA).peekable()
        }
    }

    fn read_register(&self, cpu: &LR35902Emulator, register: LCDRegister) -> u8
    {
        cpu.read_memory(register as u16)
    }

    fn set_register(&self, cpu: &mut LR35902Emulator, register: LCDRegister, value: u8)
    {
        cpu.set_memory(register as u16, value);
    }

    fn read_lcd_control_flag(&self, cpu: &LR35902Emulator, flag: LCDControlFlag) -> bool
    {
        let lcdc = self.read_register(cpu, LCDRegister::LCDC);
        return lcdc & flag as u8 == flag as u8;
    }

    fn set_lcd_control_flag(&self, cpu: &mut LR35902Emulator, flag: LCDControlFlag, value: bool)
    {
        let lcdc = self.read_register(cpu, LCDRegister::LCDC);
        if value {
            self.set_register(cpu, LCDRegister::LCDC, lcdc | flag as u8);
        } else {
            self.set_register(cpu, LCDRegister::LCDC, lcdc & !(flag as u8));
        }
    }

    /*
    fn read_lcd_status_flag(&self, cpu: &LR35902Emulator, flag: LCDStatusFlag) -> bool
    {
        let stat = self.read_register(cpu, LCDRegister::STAT);
        return stat & flag as u8 == flag as u8;
    }
    */

    fn set_lcd_status_flag(&self, cpu: &mut LR35902Emulator, flag: LCDStatusFlag, value: bool)
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

    fn initialize_flags(&self, cpu: &mut LR35902Emulator)
    {
        self.set_lcd_control_flag(cpu, LCDControlFlag::OperationStop, true);
        self.set_lcd_control_flag(cpu, LCDControlFlag::BGCharacterDataSelection, true);
        self.set_lcd_control_flag(cpu, LCDControlFlag::BGDisplayOn, true);

        self.set_register(cpu, LCDRegister::STAT, 0x85);
    }

    fn get_scroll_origin_relative_to_lcd(&self, cpu: &LR35902Emulator) -> (i32, i32)
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

    fn get_window_origin_relative_to_lcd(&self, cpu: &LR35902Emulator) -> (i32, i32)
    {
        let x = self.read_register(cpu, LCDRegister::WX) as i32 * -1;
        let y = self.read_register(cpu, LCDRegister::WY) as i32 * -1;

        return (x, y);
    }

    fn crashed(&self) -> bool
    {
        self.crashed
    }

    fn draw_one_line(&mut self, cpu: &mut LR35902Emulator)
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
                    self.crashed = true;
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

    fn process_interrupts(&mut self, cpu: &mut LR35902Emulator)
    {
        let ly = self.read_register(cpu, LCDRegister::LY);

        // XXX This needs to live elsewhere
        let if_register = cpu.read_memory(0xFF0F);
        let ie_register = cpu.read_memory(0xFFFF);

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

struct GameBoyEmulator<'a> {
    cpu: LR35902Emulator,
    lcd_controller: LCDController<'a>,
}

impl<'a> GameBoyEmulator<'a> {
    fn new() -> GameBoyEmulator<'a> {
        GameBoyEmulator {
            cpu: LR35902Emulator::new(),
            lcd_controller: LCDController::new(),
        }
    }

    fn load_rom(&mut self, rom: &[u8])
    {
        self.cpu.load_rom(rom);
    }

    fn crashed(&self) -> bool
    {
        self.cpu.crashed() || self.lcd_controller.crashed()
    }

    fn run(&mut self)
    {
        self.lcd_controller.initialize_flags(&mut self.cpu);
        let mut last_draw = time::SystemTime::now();
        while !self.crashed() {
            self.cpu.run_one_instruction();

            // It takes the DMG LCD roughly 10ms to draw one horizontal line.
            if time::SystemTime::now().duration_since(last_draw).unwrap() >=
                time::Duration::new(0, 100000) {
                self.lcd_controller.draw_one_line(&mut self.cpu);
                self.lcd_controller.process_interrupts(&mut self.cpu);
                last_draw = time::SystemTime::now();
            }
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
