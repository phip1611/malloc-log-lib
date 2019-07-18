/// Malloc-Module that encapsulate the Function-Type-Definition of malloc
/// and provide a function to receive the callable real malloc function.

use crate::c_utils;

/// LibC-Malloc-Function-Type
pub type LibCFreeT = fn(*const libc::c_void);

static DBG_MSG: &str = "Trying to get reference to libc-free. Otherwise exiting program!\n";

pub fn get_real_free() -> LibCFreeT {
    // C-style string for symbol-name
    let symbol_c_string: c_utils::CString = "free\0".as_ptr() as c_utils::CString;

    unsafe {
        libc::write(
            libc::STDERR_FILENO,
            DBG_MSG.as_ptr() as *const libc::c_void,
            DBG_MSG.len()
        )
    };

    // Void-Pointer to address of symbol
    let real_free_addr: c_utils::CVoidPtr = unsafe { libc::dlsym(libc::RTLD_NEXT, symbol_c_string) };

    // transmute: "Reinterprets the bits of a value of one type as another type"
    // Transform void-pointer-type to callable C-Function
    return unsafe { std::mem::transmute(real_free_addr) };
}
