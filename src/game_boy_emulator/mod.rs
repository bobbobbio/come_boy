extern crate sdl2;

use std::time;

use lr35902_emulator::LR35902Emulator;

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

impl<'a> LCDController<'a> {
    fn new() -> LCDController<'a>
    {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("come boy", 512, 512)
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
            crashed: false
        }
    }

    fn crashed(&self) -> bool
    {
        self.crashed
    }

    fn draw_screen(&mut self, cpu: &mut LR35902Emulator)
    {
        /*
         * Super jankey drawing of the screen. This code doesn't belong here, and needs to be
         * rewritten. For now it is useful to get some visual output.
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

        let mut start_x = cpu.read_memory(0xFF43) as i32 * -2;
        let mut start_y = cpu.read_memory(0xFF42) as i32 * -2;

        if start_x < -256 {
            start_x += 512;
        }

        if start_y < -256 {
            start_y += 512;
        }

        for c in 0x9800..0x9C00 {
            let tile_number = cpu.read_memory(c);
            let tile_location = tile_number as u16 * 16 + 0x8000;
            let mut tile = [0u8; 64];
            for j in 0..64 {
                let byte1 = cpu.read_memory(tile_location + (j / 8) * 2);
                let byte2 = cpu.read_memory(tile_location + (j / 8) * 2 + 1);
                tile[j as usize] =
                    ((byte1 >> 7 - (j % 8)) << 1) & 0x2 | (byte2 >> 7 - (j % 8)) & 0x1;
            }

            for x in 0..8 {
                for y in 0..8 {
                    let rect = sdl2::rect::Rect::new(
                        (start_x + (((c - 0x9800) % 32) * 8 + x) as i32) * 2,
                        (start_y + (((c - 0x9800) / 32) * 8 + y) as i32) * 2, 2, 2);
                    let color = tile[(x + y * 8) as usize];
                    self.renderer.set_draw_color(match color {
                        0x0 => sdl2::pixels::Color::RGB(255, 255, 255),
                        0x1 => sdl2::pixels::Color::RGB(105, 150, 150),
                        0x2 => sdl2::pixels::Color::RGB(50, 50, 50),
                        0x3 => sdl2::pixels::Color::RGB(0, 0, 0),
                        0x4 => sdl2::pixels::Color::RGB(0, 0, 0),
                        _ => panic!("color?!")
                    });
                    self.renderer.fill_rect(rect).unwrap();
                }
            }
        }

        self.renderer.present();
    }
}

struct EmulatorGameBoy<'a> {
    cpu: LR35902Emulator,
    lcd_controller: LCDController<'a>,
}

impl<'a> EmulatorGameBoy<'a> {
    fn new() -> EmulatorGameBoy<'a> {
        EmulatorGameBoy {
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
        let mut last_draw = time::SystemTime::now();
        while !self.crashed() {
            self.cpu.run_one_instruction();

            // Draw the screen 60 times a second
            if time::SystemTime::now().duration_since(last_draw).unwrap() >=
                time::Duration::from_millis(16) {
                self.lcd_controller.draw_screen(&mut self.cpu);
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
    let mut e = EmulatorGameBoy::new();
    e.load_rom(&rom);
    e.run();
}
