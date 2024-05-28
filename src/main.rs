mod sdl_system;
mod computer;
mod memory;
mod display;
mod stack;

use std::{path::Path, time::Duration};

use computer::Computer;
use sdl2::{event::Event, keyboard::Keycode};

use crate::sdl_system::SdlSystem;



pub struct Context<'a> {
    pub sdl: &'a mut SdlSystem,
}

pub fn run(sdl: &mut SdlSystem, computer: &mut Computer) -> Result<(), String> {
    let mut event_pump = sdl.sdl_context.event_pump()?;

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
            }
        }

        {
            let mut context = Context{ sdl };
            //let current_scene = &mut self.scenes[self.current_scene_idx];
            //current_scene.update(&mut context);
            //current_scene.draw(&mut context);
            computer.update();
            computer.draw(sdl);
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let mut sdl = SdlSystem::new("Rust Chip-8", 640, 320);
    let mut computer = Computer::new();
    computer.load_program_from_file(Path::new("roms/test_opcode.ch8"));
    run(&mut sdl, &mut computer)
}
