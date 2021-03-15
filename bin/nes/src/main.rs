#![allow(clippy::identity_op)]

use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{self, quit, EventHandler, KeyCode},
    graphics::{self, Color, DrawParam, Font, Image, Scale, Text, TextFragment},
    timer, Context, ContextBuilder, GameResult,
};
use nes::{
    cpu6502::{Cpu6502, Flags},
    emulator::Emulator,
    ppu2C02::{Ppu2C02, SCREEN_HEIGHT, SCREEN_WIDTH},
};
use std::{collections::HashMap, env};
use utils::prelude::*;

const WIDTH: f32 = 960.0;
const HEIGHT: f32 = 540.0;

fn main() -> GameResult<()> {
    utils::init_logger().unwrap();

    let (mut ctx, mut event_loop) = ContextBuilder::new("nes_emulator", "remtori")
        .window_setup(WindowSetup::default().title("NES Emulator"))
        .window_mode(
            WindowMode::default()
                .dimensions(WIDTH, HEIGHT)
                .resizable(true),
        )
        .add_resource_path(env::current_dir()?.join("res"))
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
}

impl App {
    pub fn new(ctx: &mut Context) -> GameResult<App> {
        let font = Font::new(ctx, "/CascadiaMono.ttf")?;
        let mut emulator = {
            let mut nes = Emulator::default();
            nes.reset();

            nes
        };

        let disassembly = emulator.disassemble(0x0000..0xFFFF);

        Ok(App {
            font,
            is_step_mode: true,
            disassembly,
            emulator,
        })
    }
}

impl EventHandler for App {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.is_step_mode {
            self.emulator.tick();
        }

        // timer::sleep(timer::remaining_update_time(ctx));
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::from_rgb_u32(0x252525));

        let mut stats = Text::new("STATUS: ");
        stats.set_font(self.font, Scale::uniform(18.0));

        let cpu = self.emulator.cpu();
        let pc = cpu.program_counter();

        {
            macro_rules! color {
                ($flag: ident) => {
                    Color::from_rgb_u32(if cpu.flag(Flags::$flag) > 0 {
                        0x00ff00
                    } else {
                        0xffffff
                    })
                };
            }

            stats.add(TextFragment::new("N ").color(color!(N)));
            stats.add(TextFragment::new("V ").color(color!(V)));
            stats.add(TextFragment::new("- ").color(color!(U)));
            stats.add(TextFragment::new("B ").color(color!(B)));
            stats.add(TextFragment::new("D ").color(color!(D)));
            stats.add(TextFragment::new("I ").color(color!(I)));
            stats.add(TextFragment::new("Z ").color(color!(Z)));
            stats.add(TextFragment::new("C ").color(color!(C)));
            stats.add("\n");
        }

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
            SCREEN_WIDTH as u16,
            SCREEN_HEIGHT as u16,
            self.emulator.ppu().screen(),
        )?;
        graphics::draw(ctx, &img, DrawParam::default().scale([2.0, 2.0]))?;

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
