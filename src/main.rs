mod sdl_system;
mod computer;
mod memory;
mod display;
mod stack;
mod instruction;
mod keyboard;
mod timer;

use std::{path::Path, time::Duration};
use clap::Parser;

use computer::Computer;
use keyboard::Keyboard;
use sdl2::{event::Event, keyboard::Keycode};

use crate::sdl_system::SdlSystem;

/// Chip-8 Emulator written in rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to the ROM file to read
    path: std::path::PathBuf,
}

pub struct Context<'a> {
    pub sdl: &'a mut SdlSystem,
}

pub fn run(sdl: &mut SdlSystem, computer: &mut Computer) -> Result<(), String> {
    let mut event_pump = sdl.sdl_context.event_pump()?;
    let mut keyboard = Keyboard::new();

    'running: loop {
        {
            //let mut context = Context{ sdl };
            
            // Handle events
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running;
                    },
                    _ => {}
                }

                //let current_scene = &mut self.scenes[self.current_scene_idx];
                //current_scene.process_event(&mut context, event);

                keyboard.process_event(event);
            }
        }

        {
            //let mut context = Context{ sdl };
            let dt = 0.0167f32;
            keyboard.update();
            //let current_scene = &mut self.scenes[self.current_scene_idx];
            //current_scene.update(&mut context);
            //current_scene.draw(&mut context);
            computer.update(dt, &mut keyboard);
            computer.draw(sdl);
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let mut sdl = SdlSystem::new("Rust Chip-8", 640, 320);
    let mut computer = Computer::new();
    computer.load_program_from_file(Path::new(&args.path));
    run(&mut sdl, &mut computer)
}
