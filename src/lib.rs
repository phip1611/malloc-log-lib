extern crate libc;

use std::io::Write;

const MSG: &str = "HELLO WORLD\n";

#[no_mangle] // then "malloc" is the symbol name so that ELF-Files can find it (if this lib is preloaded)
// TODO: Save the whole malloc Wrapper
pub extern fn malloc(bytes: usize) -> *mut libc::c_void {
    /// Disable Logging aka return immediately
    static mut RETURN_IMMEDIATELY: bool = false;

    let c_string = "malloc\0".as_ptr() as *mut i8; // char array for libc
    let real_malloc_addr: *mut libc::c_void = unsafe {libc::dlsym(libc::RTLD_NEXT, c_string)};

    let real_malloc: extern "C" fn(usize) -> *mut libc::c_void = unsafe { std::mem::transmute(real_malloc_addr) };

    unsafe {
        if (!RETURN_IMMEDIATELY) {
            // let's do logging and other stuff that potentially
            // needs malloc() itself

            // Prevent Infinite Loop because 'std::io::stdout().write_all'
            // also uses malloc itself
            RETURN_IMMEDIATELY = true;
            std::io::stdout().write_all(MSG.as_bytes());
            RETURN_IMMEDIATELY = false
        }
    }
    //std::io::stdout().write_all("Hallo\n".as_bytes());

    (real_malloc)(bytes)

    // minimal test if other programs receive anything (comment out EE)
    //1 as *mut libc::c_void
    //real_malloc_addr as *mut libc::c_void
}
