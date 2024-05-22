
pub struct Memory {
    memory: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        let memory = [0u8; 4096];
        Self {
            memory,
        }
    }

    pub fn load(&mut self, start_address: usize, data: &[u8]) {
        // https://stackoverflow.com/questions/25225346/how-do-you-copy-between-arrays-of-different-sizes-in-rust
        let end_address = start_address + data.len();
        self.memory[start_address..end_address].copy_from_slice(&data);
    }
}