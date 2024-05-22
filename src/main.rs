mod sdl_system;

use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode};

use crate::sdl_system::SdlSystem;

pub struct Context<'a> {
    pub sdl: &'a mut SdlSystem,
}

pub fn run(sdl: &mut SdlSystem) -> Result<(), String> {
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
            //let mut context = Context{ sdl };
            //let current_scene = &mut self.scenes[self.current_scene_idx];
            //current_scene.update(&mut context);
            //current_scene.draw(&mut context);
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let mut sdl = SdlSystem::new("Rust Chip-8", 640, 480);
    run(&mut sdl)
}
