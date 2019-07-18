/// Malloc-Module that encapsulate the Function-Type-Definition of malloc
/// and provide a function to receive the callable real malloc function.

/// LibC-Malloc-Function-Type
pub type LibCMallocT = fn(usize) -> *mut libc::c_void;

pub fn get_real_malloc() -> LibCMallocT {
    // C-Style string for symbol-name
    let c_string = "malloc\0".as_ptr() as *mut i8; // char array for libc
    // Void-Pointer to address of symbol
    let real_malloc_addr: *mut libc::c_void = unsafe {libc::dlsym(libc::RTLD_NEXT, c_string)};
    // transmute: "Reinterprets the bits of a value of one type as another type"
    // Transform void-pointer-type to callable C-Function
    return unsafe { std::mem::transmute(real_malloc_addr) };
}
