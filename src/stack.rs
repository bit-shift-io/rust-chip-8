pub struct Stack {
    stack: Vec<usize>
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack: vec![],
        }
    }

    pub fn push(&mut self, value: usize) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> usize {
        self.stack.pop().unwrap()
    }
}

