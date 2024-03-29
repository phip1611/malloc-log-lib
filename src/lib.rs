extern crate libc;
#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use crate::c_utils::CVoidPtr;

// Own Modules
mod c_malloc; // => imports c_free.rs and makes all its public members under the namespace "c_malloc::" available
mod c_free;
mod c_utils; // if this doesn't stand here, c_malloc and c_free can't import c_utils ..
mod init;
mod logging;
mod macros;


pub static mut INITIALIZER: init::Initializer = init::Initializer::new();

lazy_static! {
    // #[allow(non_upper_case_globals)]
    pub static ref LOG_CONFIG: Mutex<Option<logging::LogConfig>> = Mutex::new(None);
}

#[no_mangle] // then "malloc" is the symbol name so that ELF-Files can find it (if this lib is preloaded)
pub extern fn malloc(bytes: usize) -> *mut libc::c_void {
    static mut REAL_MALLOC: Option<c_malloc::LibCMallocT> = None;
    unsafe {
        // this will be executed only on the very first call to malloc, never again
        // therefore unsafe should be fine
        // also this can(?) only be called when there's only one thread
        // (because a malloc should) always happen before there are multiple threads
        if let Option::None = REAL_MALLOC {
            REAL_MALLOC.replace(c_malloc::get_real_malloc());
        }
    }

    // can't be None, because we checked this on the very first call of malloc at the beginning
    let res: CVoidPtr = unsafe { REAL_MALLOC.unwrap()(bytes) };

    if malloc_recur_protection::get_return_immediately() {
        return res;
    }

    malloc_no_conflict!({
        // I'm REALLY not sure if I use the lock the proper way.. at least I get it work with this
        // (because I need a global object for this from where every part of the code can access
        // configuration etc.)
        unsafe {
            if !INITIALIZER.is_runtime_init_done() {
                // In Rust we don't have (AFAIK) a life before main, therefore I can't do static initialization
                // in the constructor of a class with static lifetime --> we have to do it during runtime once;
                // this is only done in malloc because I assume that there can never be a free call before a malloc call
                INITIALIZER.init();
            }
        }
    });

    malloc_no_conflict!({
        // interpret libc-Pointer as Rust Number
        //let p_as_n: usize = unsafe { std::mem::transmute(res) };
        let p_as_n: usize = res as usize;
        let p_as_s: String = format!("0x{:x}", p_as_n);
        let record = logging::Record::new_malloc(p_as_s, bytes as u64);
        logging::write_record(record);
    });

    res
}

#[no_mangle] // then "free" is the symbol name so that ELF-Files can find it (if this lib is preloaded)
pub extern fn free(ptr: *const libc::c_void) {
    static mut REAL_FREE: Option<c_free::LibCFreeT> = None;
    unsafe {
        // this will be executed only on the very first call to free, never again
        // therefore unsafe should be fine
        // In contrast to malloc free could be REALLY called when there are multiple threads
        // It should be fine if multiple threads try to set the value at the same time
        // due to it should be constant --> we don't need Synchronisation overhead
        if let Option::None = REAL_FREE {
            REAL_FREE.replace(c_free::get_real_free());
        }
    }

    // fast return as early as possible if wanted
    // I didn't managed it to cover this in the 'malloc_no_conflict!'-Macro
    // (segfaults with nested macros); this solution is easier
    if malloc_recur_protection::get_return_immediately() {
        unsafe { REAL_FREE.unwrap()(ptr); };
        return;
    }

    malloc_no_conflict!({
        // interpret libc-Pointer as Rust Number
        let p_as_n: usize = ptr as usize;
        let p_as_s: String = format!("0x{:x}", p_as_n);
        let record = logging::Record::new_free(p_as_s);;
        logging::write_record(record);
    });

    unsafe { REAL_FREE.unwrap()(ptr); };
}

mod malloc_recur_protection {
    std::thread_local! {
        // All Thread-local static Vars
        // Disable logging aka return immediately the pointer from the real malloc (libc malloc)
        static RETURN_IMMEDIATELY: std::cell::RefCell<bool> = std::cell::RefCell::new(false);
        // tells us if we are in a chain of multiple
        static IS_IN_MACRO_CHAIN: std::cell::RefCell<bool> = std::cell::RefCell::new(false);
    }

    // as mentioned here https://stackoverflow.com/a/46866917/2891595
    // it's common to write getter and setter for thread-local/LocalKey-vars
    pub fn get_return_immediately() -> bool {
        RETURN_IMMEDIATELY.with(|val| val.borrow().clone())
    }
    pub fn enable_return_immediately() {
        RETURN_IMMEDIATELY.with(|val| {
            *val.borrow_mut() = true;
        });
    }
    pub fn disable_return_immediately() {
        RETURN_IMMEDIATELY.with(|val| {
            *val.borrow_mut() = false;
        });
    }

    pub fn get_is_in_macro_chain() -> bool {
        IS_IN_MACRO_CHAIN.with(|val| val.borrow().clone())
    }
    pub fn truify_is_in_macro_chain() {
        IS_IN_MACRO_CHAIN.with(|val| {
            *val.borrow_mut() = true;
        });
    }
    pub fn falsify_is_in_macro_chain() {
        IS_IN_MACRO_CHAIN.with(|val| {
            *val.borrow_mut() = false;
        });
    }
}

