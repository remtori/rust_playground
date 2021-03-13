#![allow(clippy::identity_op)]

use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{self, quit, EventHandler, KeyCode},
    graphics::{self, Color, DrawParam, Font, Image, Scale, Text, TextFragment},
    timer, Context, ContextBuilder, GameResult,
};
use nes::{cpu6502::Cpu6502, emulator::Emulator};
use std::{collections::HashMap, env};
use utils::prelude::*;

const PROGRAM: [u8; 28] = [
    0xA2, 0x0A, 0x8E, 0x00, 0x00, 0xA2, 0x03, 0x8E, 0x01, 0x00, 0xAC, 0x00, 0x00, 0xA9, 0x00, 0x18,
    0x6D, 0x01, 0x00, 0x88, 0xD0, 0xFA, 0x8D, 0x02, 0x00, 0xEA, 0xEA, 0xEA,
];

const WIDTH: f32 = 960.0;
const HEIGHT: f32 = 540.0;
const NES_WIDTH: u16 = 256;
const NES_HEIGHT: u16 = 240;
const NES_SCALE: u16 = 2;

fn main() -> GameResult<()> {
    utils::init_logger().unwrap();

    let (mut ctx, mut event_loop) = ContextBuilder::new("nes_emulator", "remtori")
        .window_setup(WindowSetup::default().title("NES Emulator"))
        .window_mode(
            WindowMode::default()
                .dimensions(WIDTH, HEIGHT)
                .resizable(true),
        )
        .add_resource_path(env::current_dir()?.join("resources"))
        .build()
        .expect("aieee, could not create ggez context!");

    let mut app = App::new(&mut ctx)?;

    // Run!
    event::run(&mut ctx, &mut event_loop, &mut app)
    // Ok(())
}

struct App {
    font: Font,
    is_step_mode: bool,
    emulator: Emulator,
    disassembly: HashMap<u16, String>,
    pixel_buffer: Vec<u8>,
}

impl App {
    pub fn new(ctx: &mut Context) -> GameResult<App> {
        let font = Font::new(ctx, "/CascadiaMono.ttf")?;
        let mut emulator = {
            let mut nes = Emulator::default();
            nes.write_ram(0x8000, &PROGRAM);

            nes.write_ram(Cpu6502::DEFAULT_PC + 0, &[0x00]);
            nes.write_ram(Cpu6502::DEFAULT_PC + 1, &[0x80]);
            nes.reset();
            nes
        };

        let disassembly = emulator.disassemble(0x0000..0xFFFF);

        Ok(App {
            font,
            is_step_mode: true,
            disassembly,
            emulator,
            pixel_buffer: vec![
                0xcc;
                4 * (NES_WIDTH * NES_HEIGHT) as usize
                    * (NES_SCALE * NES_SCALE) as usize
            ],
        })
    }
}

impl EventHandler for App {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.is_step_mode {
            self.emulator.tick(1);
        }

        // timer::sleep(timer::remaining_update_time(ctx));
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::from_rgb_u32(0x252525));

        let mut stats = Text::new("STATUS: ");
        stats.set_font(self.font, Scale::uniform(18.0));
        stats.add(TextFragment::new("N ").color(Color::from_rgb_u32(0x00ff00)));
        stats.add(TextFragment::new("V ").color(Color::from_rgb_u32(0x00ff00)));
        stats.add(TextFragment::new("- ").color(Color::from_rgb_u32(0x00ff00)));
        stats.add(TextFragment::new("B ").color(Color::from_rgb_u32(0x00ff00)));
        stats.add(TextFragment::new("D ").color(Color::from_rgb_u32(0x00ff00)));
        stats.add(TextFragment::new("I ").color(Color::from_rgb_u32(0x00ff00)));
        stats.add(TextFragment::new("Z ").color(Color::from_rgb_u32(0x00ff00)));
        stats.add(TextFragment::new("C ").color(Color::from_rgb_u32(0x00ff00)));
        stats.add("\n");

        let cpu = self.emulator.cpu();
        let pc = cpu.program_counter();
        stats.add(format!("PC: ${:04x}\n", pc));
        stats.add(format!(
            "A: ${:02x} [{}]\n",
            cpu.register_a(),
            cpu.register_a()
        ));
        stats.add(format!(
            "X: ${:02x} [{}]\n",
            cpu.register_x(),
            cpu.register_x()
        ));
        stats.add(format!(
            "Y: ${:02x} [{}]\n",
            cpu.register_y(),
            cpu.register_y()
        ));
        stats.add(format!("Stack Ptr: ${:04x}\n\n", cpu.stack_pointer()));

        let mut offset = -12;
        for _ in 0..20 {
            loop {
                let addr = pc as i32 + offset;
                offset += 1;
                if !(0x0000..=0xFFFF).contains(&addr) {
                    continue;
                }

                let addr = addr as u16;
                if let Some(str) = self.disassembly.get(&addr) {
                    stats.add(if addr == pc {
                        TextFragment::new(&str[..]).color(Color::from_rgb_u32(0x00CCCC))
                    } else {
                        TextFragment::new(&str[..])
                    });

                    stats.add("\n");
                    break;
                }
            }
        }

        graphics::draw(ctx, &stats, ([WIDTH - 275.0, 0.0], graphics::WHITE))?;

        let img = graphics::Image::from_rgba8(
            ctx,
            NES_WIDTH * NES_SCALE,
            NES_HEIGHT * NES_SCALE,
            &self.pixel_buffer,
        )?;
        graphics::draw(ctx, &img, DrawParam::default())?;

        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Space => {
                info!("Emulator Step");
                self.emulator.step();
            }
            KeyCode::R => {
                info!("Emulator Reset!");
                self.emulator.reset();
            }

            KeyCode::Escape => event::quit(ctx),
            _ => {}
        }
    }
}
