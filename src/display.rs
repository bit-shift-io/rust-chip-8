use sdl2::{pixels::Color, rect::Point};

use crate::{memory::Memory, sdl_system::SdlSystem};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    buffer: [u8; WIDTH * HEIGHT],
    dirty: bool,
}

impl Display {
    pub fn new() -> Self {
        Self {
            buffer: [0; WIDTH * HEIGHT],
            dirty: false,
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [0; 64 * 32];
    }

    pub fn draw(&mut self, sdl: &mut SdlSystem) {
        if !self.dirty {
            return;
        }

        sdl.canvas.set_draw_color(Color::RGB(0, 0, 0));
        sdl.canvas.clear();

        let (u_width, u_height) = sdl.canvas.output_size().unwrap();
        let x_step = u_width as usize / WIDTH;
        let y_step = u_height as usize / HEIGHT;

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pixel_idx = (y * WIDTH + x) as usize;
                let pixel = self.buffer[pixel_idx];

                
                if pixel == 0 {
                    //print!("_");
                    continue;
                }
                //print!("X");

                let color = Color::RGB(255, 255, 255);
                /*
                let color = if pixel == 1 {
                    Color::RGB(255, 255, 255)
                } else {
                    Color::RGB(0, 0, 0)
                };*/

                let sdl_x = i32::try_from(x * x_step).unwrap();
                let sdl_y = i32::try_from(y * y_step).unwrap();
                
                let sdl_point = Point::new(sdl_x, sdl_y);
                let size = Point::new(x_step as i32, y_step as i32);
                sdl.draw_filled_rect(sdl_point, size, color);
            }

            //println!("");
        }

        //println!("");

        sdl.canvas.present();

        self.dirty = false;
    }

    pub fn xor_sprite(&mut self, x: u16, y: u16, num_rows: u16, memory: &Memory, index_register: usize) -> u16 {
        let xr = x & (WIDTH - 1) as u16;
        let yr = y & (HEIGHT - 1) as u16;

        let mut vf = 0;

        for row_idx in 0..num_rows {
            let sprite = memory.read_u8(index_register + row_idx as usize);
            let y = yr + row_idx;
            if y >= HEIGHT as u16 {
                return vf;
            }

            for col_idx in 0..8 {
                let x = xr + col_idx;
                if x >= WIDTH as u16 {
                    break;
                }

                let pixel_idx = (y * WIDTH as u16 + x) as usize;
                let sprite_bit = (sprite >> (7 - col_idx)) & 0x1;
                if sprite_bit == 1 && self.buffer[pixel_idx] == 1 {
                    self.buffer[pixel_idx] = 0;
                    vf = 1; // set VF to 1? overflow register? https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#dxyn-display
                } else if sprite_bit == 1 && self.buffer[pixel_idx] == 0 {
                    self.buffer[pixel_idx] = 1;
                }
            }
        }

        self.dirty = true;
        vf
    }
}