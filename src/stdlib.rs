//! stdlib.h Rust implementations

use core::ffi::c_int;

// Re-export straight from dependency
pub use tinyrlibc::qsort;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn abs(n: c_int) -> c_int {
    n.abs()
}

// FIXME: Not thoroughly checked and tested
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bsearch(
    key: *const core::ffi::c_void,
    base: *const core::ffi::c_void,
    nmemb: usize,
    size: usize,
    compar: unsafe extern "C" fn(
        *const core::ffi::c_void,
        *const core::ffi::c_void,
    ) -> core::ffi::c_int,
) -> *mut core::ffi::c_void {
    if base.is_null() || compar as usize == 0 || size == 0 {
        return core::ptr::null_mut();
    }

    let base_ptr = base as *const u8;
    let mut left = 0;
    let mut right = nmemb;

    while left < right {
        let mid = left + (right - left) / 2;
        let mid_ptr = unsafe { base_ptr.add(mid * size) as *const core::ffi::c_void };

        let cmp = unsafe { compar(key, mid_ptr) };

        if cmp == 0 {
            return mid_ptr as *mut core::ffi::c_void; /* Found */
        } else if cmp < 0 {
            right = mid; /* Search left half */
        } else {
            left = mid + 1; /* Search right half */
        }
    }

    core::ptr::null_mut() /* Not found */
}
