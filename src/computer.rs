
use crate::memory::Memory;
use crate::display::Display;
use crate::instruction::Instruction;
use crate::sdl_system::SdlSystem;
use crate::stack::Stack;

use std::fs;
use std::path::Path;

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

const FONT_MEMORY_START: usize = 0x50;
const ROM_START: usize = 0x200;

const OP_CODE_MASK: u16 = 0xF000;

pub struct Computer {
    memory: Memory,
    display: Display,
    stack: Stack,

    program_counter: usize,
    index_register: usize,
    registers: [u8; 16],
}

impl Computer {
    pub fn new() -> Self {
        let mut memory = Memory::new();
        memory.load(FONT_MEMORY_START, &FONT);

        Self {
            memory,
            display: Display::new(),
            stack: Stack::new(),
            program_counter: 0,
            index_register: 0,
            registers: [0; 16],
        }
    }

    pub fn load_program_from_file(&mut self, path: &Path) {
        let data: Vec<u8> = fs::read(path).unwrap();
        self.memory.load(ROM_START, &data);
        self.program_counter = ROM_START;
    }

    pub fn update(&mut self) {
        // fetch instruction
        let instruction = Instruction::new(self.memory.read_u16(self.program_counter));
        self.program_counter += 2;

        // decode & execute
        let opcode = instruction.op_code();
        match opcode {
            0x0 => self.op_00e0_clear_screen(instruction),
            0xE => self.op_00ee_return_from_subroutine(instruction),
            0xA => self.op_annn_set_index_register(instruction),
            0xD => self.op_dxyn_display(instruction),
            0x1 => self.op_1nnn_jump(instruction),
            0x2 => self.op_2nnn_call_subroutine(instruction),
            0x3 => self.op_3xnn_skip_if_equal(instruction),
            0x4 => self.op_4xnn_skip_if_not_equal(instruction),
            0x5 => self.op_5xy0_skip_if_registers_equal(instruction),
            0x6 => self.op_6xnn_set_register(instruction),
            0x7 => self.op_7xnn_add_register(instruction),
            0x8 => {
                let lsb = instruction.n(); //instruction & 0x000F;
                match lsb {
                    0x0 => self.op_8xy0_set(instruction),
                    0x1 => self.op_8xy1_binary_or(instruction),
                    0x2 => self.op_8xy2_binary_and(instruction),
                    0x3 => self.op_8xy3_binary_xor(instruction),
                    0x4 => self.op_8xy4_add(instruction),
                    0x5 => self.op_8xy5_subtract(instruction),
                    0x6 => self.op_8xy6_shift(instruction),
                    0x7 => self.op_8xy7_subtract(instruction),
                    0xE => self.op_8xye_shift(instruction),
                    _ => println!("Unknown 8 lsb: {:#06X}", lsb),
                }
            },
            0x9 => self.op_9xy0_skip_if_registers_not_equal(instruction),
            0xF => {
                let lsb = instruction.nn(); //instruction & 0x00FF;
                match lsb {
                    0x07 => self.op_fx07_timer(instruction),
                    0x15 => self.op_fx15_timer(instruction),
                    0x18 => self.op_fx18_timer(instruction),
                    0x1E => self.op_fx1e_index_register_add(instruction),
                    0x0A => self.op_fx0a_get_keyboard_input(instruction),
                    0x29 => self.op_fx29_font_character(instruction),
                    0x33 => self.op_fx29_binary_coded_decimal_conversion(instruction),
                    0x55 => self.op_fx55_store_memory(instruction),
                    0x65 => self.op_fx65_load_memory(instruction),
                    _ => println!("Unknown F lsb: {:#06X}", lsb),
                }
            },
            _ => {
                println!("Unknown opcode: {:#06X}", instruction.instruction)
            },
        }
    }

    fn op_fx65_load_memory(&mut self, instruction: Instruction) {
        println!("todo: op_fx65_load_memory")
    }

    fn op_fx55_store_memory(&mut self, instruction: Instruction) {
        println!("todo: op_fx55_store_memory")
    }

    fn op_fx29_binary_coded_decimal_conversion(&mut self, instruction: Instruction) {
        println!("todo: op_fx29_binary_coded_decimal_conversion")
    }

    fn op_fx29_font_character(&mut self, instruction: Instruction) {
        let xi = instruction.x();
        let x = self.registers[xi];
        self.index_register = FONT_MEMORY_START + (5 * x as usize);
    }

    fn op_fx0a_get_keyboard_input(&mut self, instruction: Instruction) {
        println!("todo: op_fx0a_get_keyboard_input")
    }

    fn op_fx1e_index_register_add(&mut self, instruction: Instruction) {
        println!("todo: op_fx1e_index_register_add")
    }

    fn op_fx07_timer(&mut self, instruction: Instruction) {
        println!("todo: op_fx07_timer")
    }

    fn op_fx15_timer(&mut self, instruction: Instruction) {
        println!("todo: op_fx15_timer")
    }


    fn op_fx18_timer(&mut self, instruction: Instruction) {
        println!("todo: op_fx18_timer")
    }

    fn op_8xy0_set(&mut self, instruction: Instruction) {
        println!("todo: op_8xy0_set")
    }

    fn op_8xy1_binary_or(&mut self, instruction: Instruction) {
        let [xi, yi] = instruction.xy();
        self.registers[xi] |= self.registers[yi];
    }

    fn op_8xy2_binary_and(&mut self, instruction: Instruction) {
        let [xi, yi] = instruction.xy();
        self.registers[xi] &= self.registers[yi];
    }

    fn op_8xy3_binary_xor(&mut self, instruction: Instruction) {
        let [xi, yi] = instruction.xy();
        self.registers[xi] ^= self.registers[yi];
    }

    fn op_8xy4_add(&mut self, instruction: Instruction) {
        let [xi, yi] = instruction.xy();
        self.registers[xi] &= self.registers[yi];
    }

    fn op_8xy5_subtract(&mut self, instruction: Instruction) {
        let [xi, yi] = instruction.xy();
        let x = self.registers[xi];
        let y = self.registers[yi];
        
        if x > y {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[xi] = x.wrapping_sub(y);
    }

    fn op_8xy7_subtract(&mut self, instruction: Instruction) {
        let [xi, yi] = instruction.xy();
        let x = self.registers[xi];
        let y = self.registers[yi];

        if y > x {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[xi] = y.wrapping_sub(x);
    }

    fn op_8xy6_shift(&mut self, instruction: Instruction) {
        let xi = instruction.x();

        // Save LSB in VF
	    self.registers[0xF] = self.registers[xi] & 0x1;

        self.registers[xi] >>= 1;
    }

    fn op_8xye_shift(&mut self, instruction: Instruction) {
        let xi = instruction.x();

        // Save MSB in VF
	    self.registers[0xF] = (self.registers[xi] & 0x80) >> 7;

        self.registers[xi] <<= 1;
    }

    fn op_5xy0_skip_if_registers_equal(&mut self, instruction: Instruction) {
        let [xi, yi] = instruction.xy();
        let x = self.registers[xi];
        let y = self.registers[yi];
        if x == y {
            self.program_counter += 2;
        }
    }

    fn op_9xy0_skip_if_registers_not_equal(&mut self, instruction: Instruction) {
        let [xi, yi] = instruction.xy();
        let x = self.registers[xi];
        let y = self.registers[yi];
        if x != y {
            self.program_counter += 2;
        }
    }

    fn op_3xnn_skip_if_equal(&mut self, instruction: Instruction) {
        let xi = instruction.x();
        let value = instruction.nn();
        let x = self.registers[xi];
        if x == value {
            self.program_counter += 2;
        }
    }

    fn op_4xnn_skip_if_not_equal(&mut self, instruction: Instruction) {
        let xi = instruction.x();
        let value = instruction.nn();
        let x = self.registers[xi];
        if x != value {
            self.program_counter += 2;
        }
    }

    fn op_2nnn_call_subroutine(&mut self, instruction: Instruction) {
        self.stack.push(self.program_counter);
        let address = instruction.nnn();
        self.program_counter = address as usize;
    }

    fn op_00ee_return_from_subroutine(&mut self, _instruction: Instruction) {
        self.program_counter = self.stack.pop() as usize;
    }

    fn op_00e0_clear_screen(&mut self, _instruction: Instruction) {
        self.display.clear();
    }

    fn op_annn_set_index_register(&mut self, instruction: Instruction) {
        let value = instruction.nnn();
        self.index_register = value as usize;
    }

    fn op_dxyn_display(&mut self, instruction: Instruction) {
        let [xi, yi] = instruction.xy();
        let num_rows = instruction.n();
        let x = self.registers[xi];
        let y = self.registers[yi];
        let vf = self.display.xor_sprite(x, y, num_rows, &self.memory, self.index_register);
        self.registers[0xF] = vf;
    }

    fn op_1nnn_jump(&mut self, instruction: Instruction) {
        let address = instruction.nnn();
        self.program_counter = address as usize;
    }

    fn op_6xnn_set_register(&mut self, instruction: Instruction) {
        let register = instruction.x();
        let value = instruction.nn();
        self.registers[register as usize] = value;
    }

    fn op_7xnn_add_register(&mut self, instruction: Instruction) {
        let register = instruction.x();
        let value = instruction.nn();
        self.registers[register as usize] = self.registers[register as usize].wrapping_add(value);
    }

    pub fn draw(&mut self, sdl: &mut SdlSystem) {
        self.display.draw(sdl);
    }
}

