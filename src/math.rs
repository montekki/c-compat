//! math.h Rust implementations

use core::ffi::c_double;
use core::ffi::c_float;

#[unsafe(no_mangle)]
pub extern "C" fn fabs(arg: c_double) -> c_double {
    arg.abs()
}

#[unsafe(no_mangle)]
pub extern "C" fn fabsf(arg: c_float) -> c_float {
    arg.abs()
}

#[unsafe(no_mangle)]
pub extern "C" fn fmin(x: c_double, y: c_double) -> c_double {
    x.min(y)
}

#[unsafe(no_mangle)]
pub extern "C" fn fminf(x: c_float, y: c_float) -> c_float {
    x.min(y)
}

#[unsafe(no_mangle)]
pub extern "C" fn fmax(x: c_double, y: c_double) -> c_double {
    x.max(y)
}

#[unsafe(no_mangle)]
pub extern "C" fn fmaxf(x: c_float, y: c_float) -> c_float {
    x.max(y)
}

#[unsafe(no_mangle)]
pub extern "C" fn sqrt(x: c_double) -> c_double {
    libm::sqrt(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn sqrtf(x: c_float) -> c_float {
    libm::sqrtf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn ceil(x: c_double) -> c_double {
    libm::ceil(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn ceilf(x: c_float) -> c_float {
    libm::ceilf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn floor(x: c_double) -> c_double {
    libm::floor(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn floorf(x: c_float) -> c_float {
    libm::floorf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn trunc(x: c_double) -> c_double {
    libm::trunc(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn truncf(x: c_float) -> c_float {
    libm::truncf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn rint(x: c_double) -> c_double {
    libm::rint(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn rintf(x: c_float) -> c_float {
    libm::rintf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn isnan_f32(x: c_float) -> bool {
    x.is_nan()
}

#[unsafe(no_mangle)]
pub extern "C" fn isnan_f64(x: c_double) -> bool {
    x.is_nan()
}

#[unsafe(no_mangle)]
pub extern "C" fn signbit_f32(x: c_float) -> bool {
    x.is_sign_negative()
}

#[unsafe(no_mangle)]
pub extern "C" fn signbit_f64(x: c_double) -> bool {
    x.is_sign_negative()
}
