extern crate libc;

use std::io::Write;


std::thread_local! {
    // All Thread-local static Vars
    // Disable logging aka return immediately the pointer from the real malloc (libc malloc)
    static RETURN_IMMEDIATELY: std::cell::RefCell<bool> = std::cell::RefCell::new(false);
}

const MSG: &str = "HELLO WORLD\n";

type LibCMallocT = fn(usize) -> *mut libc::c_void;

#[no_mangle] // then "malloc" is the symbol name so that ELF-Files can find it (if this lib is preloaded)
pub extern fn malloc(bytes: usize) -> *mut libc::c_void {

    // C-Style string for symbol-name
    let c_string = "malloc\0".as_ptr() as *mut i8; // char array for libc
    // Void-Pointer to address of symbol
    let real_malloc_addr: *mut libc::c_void = unsafe {libc::dlsym(libc::RTLD_NEXT, c_string)};
    // transmute: "Reinterprets the bits of a value of one type as another type"
    // Transform void-pointer-type to callable C-Function
    let real_malloc: LibCMallocT = unsafe { std::mem::transmute(real_malloc_addr) };

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

    real_malloc(bytes)
}


// as mentioned here https://stackoverflow.com/a/46866917/2891595
// it's common to write getter and setter for thread-local LocalKey-vars
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
