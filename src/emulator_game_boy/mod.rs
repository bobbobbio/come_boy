extern crate sdl2;

use self::sdl2::event::Event;
use self::sdl2::pixels::Color;
use self::sdl2::rect::Rect;
use self::sdl2::render::Renderer;

use std::time;

use emulator_lr35902::EmulatorLR35902;

struct EmulatorGameBoy {
    cpu: EmulatorLR35902
}

impl EmulatorGameBoy {
    fn new() -> EmulatorGameBoy {
        EmulatorGameBoy {
            cpu: EmulatorLR35902::new()
        }
    }

    fn load_rom(&mut self, rom: &[u8])
    {
        self.cpu.load_rom(rom);
    }

    fn draw_screen(&self, renderer: &mut Renderer)
    {
        /*
         * Super jankey drawing of the screen. This code doesn't belong here, and needs to be
         * rewritten. For now it is useful to get some visual output.
         */
        renderer.set_draw_color(Color::RGB(255, 255, 255));
        renderer.clear();

        let mut start_x = self.cpu.read_memory(0xFF43) as i32 * -2;
        let mut start_y = self.cpu.read_memory(0xFF42) as i32 * -2;

        if start_x < -256 {
            start_x += 512;
        }

        if start_y < -256 {
            start_y += 512;
        }

        for c in 0x9800..0x9C00 {
            let tile_number = self.cpu.read_memory(c);
            let tile_location = tile_number as u16 * 16 + 0x8000;
            let mut tile = [0u8; 64];
            for j in 0..64 {
                let byte1 = self.cpu.read_memory(tile_location + (j / 8) * 2);
                let byte2 = self.cpu.read_memory(tile_location + (j / 8) * 2 + 1);
                tile[j as usize] =
                    ((byte1 >> 7 - (j % 8)) << 1) & 0x2 | (byte2 >> 7 - (j % 8)) & 0x1;
            }

            for x in 0..8 {
                for y in 0..8 {
                    let rect = Rect::new(
                        (start_x + (((c - 0x9800) % 32) * 8 + x) as i32) * 2,
                        (start_y + (((c - 0x9800) / 32) * 8 + y) as i32) * 2, 2, 2);
                    let color = tile[(x + y * 8) as usize];
                    match color {
                        0x0 => renderer.set_draw_color(Color::RGB(255, 255, 255)),
                        0x1 => renderer.set_draw_color(Color::RGB(105, 150, 150)),
                        0x2 => renderer.set_draw_color(Color::RGB(50, 50, 50)),
                        0x3 => renderer.set_draw_color(Color::RGB(0, 0, 0)),
                        0x4 => renderer.set_draw_color(Color::RGB(0, 0, 0)),
                        _ => panic!("color?!")
                    }
                    renderer.fill_rect(rect).unwrap();
                }
            }
        }

        renderer.present();
    }

    fn run(&mut self)
    {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("come boy", 512, 512)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut renderer = window.renderer().build().unwrap();

        renderer.set_draw_color(Color::RGB(255, 255, 255));
        renderer.clear();
        renderer.present();

        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut last_draw = time::SystemTime::now();
        'running: while !self.cpu.crashed() {
            self.cpu.run_one_instruction();

            // Draw the screen 60 times a second
            if time::SystemTime::now().duration_since(last_draw).unwrap() >=
                time::Duration::from_millis(16) {
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit {..} => {
                            break 'running;
                        },
                        _ => {}
                    }
                }
                self.draw_screen(&mut renderer);
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
