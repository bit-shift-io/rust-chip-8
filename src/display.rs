pub struct Display {
    buffer: [u8; 64 * 32],
}

impl Display {
    pub fn new() -> Self {
        Self {
            buffer: [0; 64 * 32],
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [0; 64 * 32];
    }
}