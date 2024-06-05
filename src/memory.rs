
pub struct Memory {
    memory: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        let memory = [0; 4096];
        Self {
            memory,
        }
    }

    pub fn load(&mut self, start_address: usize, data: &[u8]) {
        // https://stackoverflow.com/questions/25225346/how-do-you-copy-between-arrays-of-different-sizes-in-rust
        let end_address = start_address + data.len();
        self.memory[start_address..end_address].copy_from_slice(&data);
    }

    pub fn read_u8_array(&self, address: usize, output: &mut [u8]) {
        let end_address = address + output.len();
        output.copy_from_slice(&self.memory[address..end_address]);
    }

    pub fn read_u8(&self, address: usize) -> u8 {
        let mut bytes = [0; 1];
        self.read_u8_array(address, &mut bytes);
        bytes[0]
    }

    pub fn read_u16(&self, address: usize) -> u16 {
        let mut bytes = [0; 2];
        self.read_u8_array(address, &mut bytes);
        u16::from_be_bytes(bytes)
    }

    pub fn write_u8(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
    }
}