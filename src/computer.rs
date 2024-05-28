
use crate::memory::Memory;
use crate::display::Display;
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
        let instruction = self.memory.read_u16(self.program_counter);
        self.program_counter += 2;

        // decode & execute
        let opcode = OP_CODE_MASK & instruction;
        match opcode {
            0x0000 => self.op_00e0_clear_screen(instruction),
            0xE000 => self.op_00ee_return_from_subroutine(instruction),
            0xA000 => self.op_annn_set_index_register(instruction),
            0xD000 => self.op_dxyn_display(instruction),
            0x1000 => self.op_1nnn_jump(instruction),
            0x2000 => self.op_2nnn_call_subroutine(instruction),
            0x3000 => self.op_3xnn_skip_if_equal(instruction),
            0x4000 => self.op_4xnn_skip_if_not_equal(instruction),
            0x5000 => self.op_5xy0_skip_if_registers_equal(instruction),
            0x6000 => self.op_6xnn_set_register(instruction),
            0x7000 => self.op_7xnn_add_register(instruction),
            0x8000 => {
                let lsb = instruction & 0x000F;
                match lsb {
                    0x0 => self.op_8xy0_set(instruction),
                    0x1 => self.op_8xy1_binary_or(instruction),
                    0x2 => self.op_8xy2_binary_and(instruction),
                    0x3 => self.op_8xy3_binary_xor(instruction),
                    0x4 => self.op_8xy4_add(instruction),
                    0x5 => self.op_8xy5_subtract(instruction),
                    0x6 => self.op_8xy6_shift(instruction),
                    0x7 => self.op_8xy7_subtract(instruction),
                    0xE => self.op_8xyE_shift(instruction),
                    _ => println!("Unknown 8 lsb: {:#06X}", lsb),
                }
            },
            0x9000 => self.op_9xy0_skip_if_registers_not_equal(instruction),
            _ => {
                println!("Unknown opcode: {:#06X}", instruction)
            },
        }
    }

    fn op_8xy0_set(&mut self, instruction: u16) {
        println!("todo:")
    }

    fn op_8xy1_binary_or(&mut self, instruction: u16) {
        println!("todo:")
    }

    fn op_8xy2_binary_and(&mut self, instruction: u16) {
        println!("todo:")
    }

    fn op_8xy3_binary_xor(&mut self, instruction: u16) {
        println!("todo:")
    }

    fn op_8xy4_add(&mut self, instruction: u16) {
        println!("todo:")
    }

    fn op_8xy5_subtract(&mut self, instruction: u16) {
        println!("todo:")
    }

    fn op_8xy7_subtract(&mut self, instruction: u16) {
        println!("todo:")
    }

    fn op_8xy6_shift(&mut self, instruction: u16) {
        println!("todo:")
    }

    fn op_8xyE_shift(&mut self, instruction: u16) {
        println!("todo:")
    }

    fn op_5xy0_skip_if_registers_equal(&mut self, instruction: u16) {
        let x_reg_idx = (instruction & 0x0F00) >> 8;
        let y_reg_idx = (instruction & 0x00F0) >> 4;
        let x = self.registers[x_reg_idx as usize];
        let y = self.registers[y_reg_idx as usize];
        if x == y {
            self.program_counter += 2;
        }
    }

    fn op_9xy0_skip_if_registers_not_equal(&mut self, instruction: u16) {
        let x_reg_idx = (instruction & 0x0F00) >> 8;
        let y_reg_idx = (instruction & 0x00F0) >> 4;
        let x = self.registers[x_reg_idx as usize];
        let y = self.registers[y_reg_idx as usize];
        if x != y {
            self.program_counter += 2;
        }
    }

    fn op_3xnn_skip_if_equal(&mut self, instruction: u16) {
        let x_reg_idx = (instruction & 0x0F00) >> 8;
        let value = (instruction & 0x00FF) as u8;
        let x = self.registers[x_reg_idx as usize];
        if x == value {
            self.program_counter += 2;
        }
    }

    fn op_4xnn_skip_if_not_equal(&mut self, instruction: u16) {
        let x_reg_idx = (instruction & 0x0F00) >> 8;
        let value = (instruction & 0x00FF) as u8;
        let x = self.registers[x_reg_idx as usize];
        if x != value {
            self.program_counter += 2;
        }
    }

    fn op_2nnn_call_subroutine(&mut self, instruction: u16) {
        self.stack.push(self.program_counter);
        let address = instruction & 0x0FFF;
        self.program_counter = address as usize;
    }

    fn op_00ee_return_from_subroutine(&mut self, _instruction: u16) {
        self.program_counter = self.stack.pop() as usize;
    }

    fn op_00e0_clear_screen(&mut self, _instruction: u16) {
        self.display.clear();
    }

    fn op_annn_set_index_register(&mut self, instruction: u16) {
        let value = instruction & 0x0FFF;
        self.index_register = value as usize;
    }

    fn op_dxyn_display(&mut self, instruction: u16) {
        let x_reg_idx = (instruction & 0x0F00) >> 8;
        let y_reg_idx = (instruction & 0x00F0) >> 4;
        let num_rows = (instruction & 0xF) as u8;
        let x = self.registers[x_reg_idx as usize];
        let y = self.registers[y_reg_idx as usize];
        //let mut sprite: [u8; 16] = [0; 16];
        //self.memory.read_u8_array(self.index_register, &sprite);
        let vf = self.display.xor_sprite(x, y, num_rows, &self.memory, self.index_register);
        self.registers[0xF] = vf;
    }

    fn op_1nnn_jump(&mut self, instruction: u16) {
        let address = !OP_CODE_MASK & instruction;
        self.program_counter = address as usize;
    }

    fn op_6xnn_set_register(&mut self, instruction: u16) {
        let register = (instruction & 0x0F00) >> 8;
        let value = (instruction & 0x00FF) as u8;
        self.registers[register as usize] = value;
    }

    fn op_7xnn_add_register(&mut self, instruction: u16) {
        let register = (instruction & 0x0F00) >> 8;
        let value = (instruction & 0x00FF) as u8;
        self.registers[register as usize] = self.registers[register as usize].wrapping_add(value);
    }

    pub fn draw(&mut self, sdl: &mut SdlSystem) {
        self.display.draw(sdl);
    }
}

