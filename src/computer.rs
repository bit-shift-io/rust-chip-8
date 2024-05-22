
use crate::memory::Memory;
use crate::display::Display;
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
const OP_A : u16 = 0xA000;
const OP_E : u16 = 0xE000;
const OP_D : u16 = 0xD000;
const OP_0 : u16 = 0x0000;
const OP_1 : u16 = 0x1000;
const OP_2 : u16 = 0x2000;
const OP_6 : u16 = 0x6000;
const OP_7 : u16 = 0x7000;

const OP_CLS: u16 = 0xE000;
const OP_RET: u16 = 0xEE00;
const OP_JP: u16 = 0x1000;
const OP_SET_VX: u16 = 0x6000;
const OP_ADD_VX: u16 = 0x7000;
const OP_SET_I: u16 = 0xA000;
const OP_DXYN: u16 = 0xD000;


pub struct Computer {
    memory: Memory,
    display: Display,
    stack: Stack,

    program_counter: usize,
    index_register: usize,
    registers: [u16; 16],
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
            OP_0 => {
                self.cls(instruction)
            },

            OP_E => {
                self.return_from_subroutine(instruction)
            },

            OP_A => {
                self.set_index_register(instruction)
            },

            OP_D => {
                self.display(instruction)
            },

            OP_1 => {
                self.jump(instruction)
            },

            OP_2 => {
                self.call_subroutine(instruction)
            },

            OP_6 => {
                self.set_register(instruction);
            },

            OP_7 => {
                self.add_register(instruction);
            }
            
            _ => println!("Unknown opcode: {:#06X}", instruction),
        }
    }

    fn call_subroutine(&mut self, instruction: u16) {
        self.stack.push(self.program_counter);
        let address = instruction & 0x0FFF;
        self.program_counter = address as usize;
    }

    fn return_from_subroutine(&mut self, _instruction: u16) {
        self.program_counter = self.stack.pop() as usize;
    }

    fn cls(&mut self, _instruction: u16) {
        self.display.clear();
    }

    fn set_index_register(&mut self, instruction: u16) {
        let value = instruction & 0x0FFF;
        self.index_register = value as usize;
    }

    fn display(&mut self, _instruction: u16) {
        println!("todo: display")
    }

    fn jump(&mut self, instruction: u16) {
        let address = !OP_CODE_MASK & instruction;
        self.program_counter = address as usize;
    }

    fn set_register(&mut self, instruction: u16) {
        let register = (instruction & 0x0F00) >> 8;
        let value = instruction & 0x00FF;
        self.registers[register as usize] = value;
    }

    fn add_register(&mut self, instruction: u16) {
        let register = (instruction & 0x0F00) >> 8;
        let value = instruction & 0x00FF;
        self.registers[register as usize] += value;
    }
}

