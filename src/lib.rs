extern crate libc;

use std::io::Write;

#[no_mangle] // then "malloc" is the symbol name so that ELF-Files can find it (if this lib is preloaded)
pub extern fn malloc(bytes: usize) -> *mut libc::c_void {
    let c_string = "malloc\0".as_ptr() as *mut i8; // char array for libc
    let real_malloc_addr: *mut libc::c_void = unsafe {libc::dlsym(libc::RTLD_NEXT, c_string)};

    let real_malloc: extern "C" fn(usize) -> *mut libc::c_void = unsafe { std::mem::transmute(real_malloc_addr) };

    std::io::stdout().write_all("Hallo\n".as_bytes());

    (real_malloc)(bytes)
}
