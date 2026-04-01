//! ctype.h Rust implementations

use core::ffi::c_int;

// Re-export from tinyrlibc where available
pub use tinyrlibc::{isalpha, isdigit, isspace, isupper};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn isalnum(c: c_int) -> c_int {
    let c = c as u8;
    (c.is_ascii_alphanumeric()) as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn isgraph(c: c_int) -> c_int {
    let c = c as u8;
    (c > 0x20 && c < 0x7f) as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn isprint(c: c_int) -> c_int {
    let c = c as u8;
    (c >= 0x20 && c < 0x7f) as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn isxdigit(c: c_int) -> c_int {
    let c = c as u8;
    (c.is_ascii_hexdigit()) as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tolower(c: c_int) -> c_int {
    let c = c as u8;
    c.to_ascii_lowercase() as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn toupper(c: c_int) -> c_int {
    let c = c as u8;
    c.to_ascii_uppercase() as c_int
}
