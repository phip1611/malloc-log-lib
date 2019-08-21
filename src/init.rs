// Make the macro available in this file!
use crate::malloc_recur_protection;
use crate::malloc_no_conflict;
use crate::logging;

pub struct Initializer {
    pub done: bool
}

impl Initializer {
    pub const fn new() -> Initializer {
        Initializer {
            done: false
        }
    }

    pub fn init(&mut self) {
        self.done = true;
        crate::LOG_CONFIG.lock().unwrap().replace(crate::logging::LogConfig::new());
        malloc_no_conflict!(
            println!("LOG_CONFIG {:#?}", crate::LOG_CONFIG.lock().unwrap())
        );
        logging::write_head();
    }
}
