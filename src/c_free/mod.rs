/// Malloc-Module that encapsulate the Function-Type-Definition of malloc
/// and provide a function to receive the callable real malloc function.

use crate::c_utils;

/// LibC-Malloc-Function-Type
pub type LibCFreeT = fn(*const libc::c_void);

pub fn get_real_free() -> LibCFreeT {
    // C-style string for symbol-name
    let symbol_c_string: c_utils::CString = "free\0".as_ptr() as c_utils::CString;
    // Void-Pointer to address of symbol
    let real_free_addr: c_utils::CVoidPtr = unsafe { libc::dlsym(libc::RTLD_NEXT, symbol_c_string) };
    // transmute: "Reinterprets the bits of a value of one type as another type"
    // Transform void-pointer-type to callable C-Function
    return unsafe { std::mem::transmute(real_free_addr) };
}
