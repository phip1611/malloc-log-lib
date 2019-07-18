use crate::logging::LogConfig;

use crate::endless_recur_protection;
use crate::malloc_no_conflict;

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

        // info: crate::LOG_CONFIG get's initialized on the first call on it (because it's lazy)
        // but anyway, we manually initialize it
        crate::LOG_CONFIG.lock().unwrap().replace(crate::logging::LogConfig::new());
        // because this should be seen as a singleton this only is called once
        // and at the line where this is called there is already a malloc_no_conflict-Macro wrapped
        // malloc_no_conflict!(
            println!("LOG_CONFIG {:?}", crate::LOG_CONFIG.lock().unwrap())
        //);
    }
}
