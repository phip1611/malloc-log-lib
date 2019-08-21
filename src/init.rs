// Make the macro available in this file!
use crate::malloc_recur_protection;
use crate::malloc_no_conflict;
use crate::logging;

pub struct Initializer {
    runtime_init_done: bool
}

impl Initializer {
    pub const fn new() -> Initializer {
        Initializer {
            runtime_init_done: false
        }
    }

    pub fn init(&mut self) {
        crate::LOG_CONFIG.lock().unwrap().replace(crate::logging::LogConfig::new());
        malloc_no_conflict!(
            println!("LOG_CONFIG {:#?}", crate::LOG_CONFIG.lock().unwrap())
        );
        logging::write_head();
        self.runtime_init_done = true;
    }

    pub fn is_runtime_init_done(&self) -> bool {
        return self.runtime_init_done;
    }
}
