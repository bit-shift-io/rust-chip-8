pub struct Instruction {
    pub instruction: u16
}

impl Instruction {
    pub fn new(instruction: u16) -> Self {
        Self {
            instruction
        }
    }

    pub fn op_code(&self) -> u8 {
        let opcode = ((0xF000 & self.instruction) >> 12) as u8;
        opcode
    }

    pub fn x(&self) -> usize {
        let xi = ((self.instruction & 0x0F00) >> 8) as usize;
        xi
    }

    pub fn y(&self) -> usize {
        let yi = ((self.instruction & 0x00F0) >> 4) as usize;
        yi
    }

    pub fn xy(&self) -> [usize; 2] {
        [self.x(), self.y()]
    }

    pub fn n(&self) -> u8 {
        let byte = (self.instruction & 0xF) as u8;
        byte
    }

    pub fn nn(&self) -> u8 {
        let byte = (self.instruction & 0x00FF) as u8;
        byte
    }

    pub fn nnn(&self) -> u16 {
        self.instruction & 0x0FFF
    }
}

