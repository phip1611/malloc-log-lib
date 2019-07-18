extern crate libc;
#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

// Own Modules
mod c_malloc; // => imports c_free.rs and makes all its public members under the namespace "c_malloc::" available
mod c_free;
mod c_utils; // if this doesn't stand here, c_malloc and c_free can't import c_utils ..
mod init;
mod logging;
mod macros;


lazy_static! {
    // #[allow(non_upper_case_globals)]
    pub static ref INITIALIZER: Mutex<init::Initializer> = Mutex::new(init::Initializer::new());
    // #[allow(non_upper_case_globals)]
    pub static ref LOG_CONFIG: Mutex<Option<logging::LogConfig>> = Mutex::new(None);
}


#[no_mangle] // then "malloc" is the symbol name so that ELF-Files can find it (if this lib is preloaded)
pub extern fn malloc(bytes: usize) -> *mut libc::c_void {
    static mut REAL_MALLOC: Option<c_malloc::LibCMallocT> = None;
    unsafe {
        // this will be executed only on the very first call to malloc, never again
        // therefore unsafe should be fine
        // also this could(?) only be called when there's only one thread
        // (because a malloc should) always happen before there are multiple
        if let Option::None = REAL_MALLOC {
            REAL_MALLOC.replace(c_malloc::get_real_malloc());
        }
    }

    // can't be None, because we checked this on the very first call of malloc at the beginning
    let res = unsafe { REAL_MALLOC.unwrap()(bytes) };

    if malloc_recur_protection::get_return_immediately() {
        return res;
    }

    malloc_no_conflict!({
        // I'm REALLY not sure if I use the lock the proper way.. at least I get it work with this
        // (because I need a global object for this from where every part of the code can access
        // configuration etc.)
        let l_init: &mut init::Initializer = &mut INITIALIZER.lock().unwrap();
        if !l_init.done {
            // In Rust we don't have (AFAIK) a life before main, therefore I can't do static initialization
            // in the constructor of a class with static lifetime --> we have to do it during runtime once;
            // this is only done in malloc because I assume that there can never be a free call before a malloc call
            eprintln!("calling .init() NOW");
            l_init.init();
            eprintln!(".init() CALLED");
        }
    });

    // Example how to use functions that need malloc/free inside this function
    /*malloc_no_conflict!(
        println!("Moin")
    );

    malloc_no_conflict!({
        println!("Moin");
        println!("Moin2");
    });*/


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

    // Example how to use functions that need malloc/free inside this function
    /*malloc_no_conflict!(
        println!("Moin")
    );

    malloc_no_conflict!({
        println!("Moin");
        println!("Moin2");
    });*/

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

