extern crate libc;

use std::io::Write;

const MSG: &str = "HELLO WORLD\n";

type LibCMallocT = fn(usize) -> *mut libc::c_void;

#[no_mangle] // then "malloc" is the symbol name so that ELF-Files can find it (if this lib is preloaded)
// TODO: Save the whole malloc Wrapper
pub extern fn malloc(bytes: usize) -> *mut libc::c_void {
    /// Disable logging aka return immediately the pointer from the real malloc (libc malloc)
    static mut RETURN_IMMEDIATELY: bool = false;

    // C-Style string for symbol-name
    let c_string = "malloc\0".as_ptr() as *mut i8; // char array for libc
    // Void-Pointer to address of symbol
    let real_malloc_addr: *mut libc::c_void = unsafe {libc::dlsym(libc::RTLD_NEXT, c_string)};
    // transmute: "Reinterprets the bits of a value of one type as another type"
    // Transform void-pointer-type to callable C-Function
    let real_malloc: LibCMallocT = unsafe { std::mem::transmute(real_malloc_addr) };

    unsafe {
        if !RETURN_IMMEDIATELY {
            // let's do logging and other stuff that potentially
            // needs malloc() itself

            // This Variable prevent infinite loops because 'std::io::stdout().write_all'
            // also uses malloc itself

            // TODO: Do proper synchronisazion
            //  (lock whole method? thread_local variable?)
            RETURN_IMMEDIATELY = true;
            match std::io::stdout().write_all(MSG.as_bytes()) {
                _ => ()
            };
            RETURN_IMMEDIATELY = false
        }
    }

    (real_malloc)(bytes)
}
