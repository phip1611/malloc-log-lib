extern crate libc;

// STD
use std::io::Write;

// Own Modules
mod c_malloc; // => imports c_malloc.rs and makes all its public members under the namespace "c_malloc::" available

std::thread_local! {
    // All Thread-local static Vars
    // Disable logging aka return immediately the pointer from the real malloc (libc malloc)
    static RETURN_IMMEDIATELY: std::cell::RefCell<bool> = std::cell::RefCell::new(false);
}

const MSG: &str = "HELLO WORLD\n";

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

    if !get_return_immediately() {
        // let's do logging and other stuff that potentially
        // needs malloc() itself

        // This Variable prevent infinite loops because 'std::io::stdout().write_all'
        // also uses malloc itself

        enable_return_immediately();
        match std::io::stdout().write_all(MSG.as_bytes()) {
            _ => ()
        };
        disable_return_immediately();
    }

    // can't be None, because we checked this on the very first call of malloc at the beginning
    unsafe { REAL_MALLOC.unwrap()(bytes) }
}


// as mentioned here https://stackoverflow.com/a/46866917/2891595
// it's common to write getter and setter for thread-local/LocalKey-vars
fn get_return_immediately() -> bool {
    RETURN_IMMEDIATELY.with(|val| val.borrow().clone())
}
fn enable_return_immediately() {
    RETURN_IMMEDIATELY.with(|val| {
        *val.borrow_mut() = true;
    });
}
fn disable_return_immediately() {
    RETURN_IMMEDIATELY.with(|val| {
        *val.borrow_mut() = false;
    });
}
