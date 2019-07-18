/// Malloc-Module that encapsulate the Function-Type-Definition of malloc
/// and provide a function to receive the callable real malloc function.

use crate::c_utils;

/// LibC-Malloc-Function-Type
pub type LibCMallocT = fn(usize) -> c_utils::CVoidPtr;

static DBG_MSG: &str = "Trying to get reference to libc-malloc. Otherwise exiting program!\n";

pub fn get_real_malloc() -> LibCMallocT {
    // C-style string for symbol-name
    let symbol_c_string:  c_utils::CString  = "malloc\0".as_ptr() as c_utils::CString;
    // Void-Pointer to address of symbol

    unsafe {
        libc::write(
            libc::STDERR_FILENO,
            DBG_MSG.as_ptr() as *const libc::c_void,
            DBG_MSG.len()
        );
    };

    // if the symbol doesn't exist (e.g. "malloca"), it seems like dlsym is again and again calling
    // (malloc which is delegating the call to) get_real_malloc.. so.. KEEP IN MIND!
    let real_malloc_addr: c_utils::CVoidPtr = unsafe { libc::dlsym(libc::RTLD_NEXT, symbol_c_string) };

    // transmute: "Reinterprets the bits of a value of one type as another type"
    // Transform void-pointer-type to callable C-Function
    return unsafe { std::mem::transmute(real_malloc_addr) };
}
