pub struct Initializer {
    pub done: bool
}

impl Initializer {
    pub fn new() -> Initializer {
        Initializer {
            done: false
        }
    }

    pub fn init(&mut self) {
        self.done = true;
    }
}
