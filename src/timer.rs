pub struct Timer {
    count: u8,
    last_tick: f32,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            count: 0,
            last_tick: 0.0,
        }
    }

    pub fn set_count(&mut self, count: u8) {
        self.count = count;
    }

    pub fn count(&self) -> u8 {
        self.count
    }

    pub fn update(&mut self, dt: f32) {
        self.last_tick += dt;

        let tick_length = 1.0 / 60.0; // 60Hz

        while self.last_tick >= tick_length {
            self.last_tick -= tick_length;

            if self.count > 0 {
                self.count -= 1;
            }
        }
    }
}