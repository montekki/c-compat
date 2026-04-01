//! c-compat
//!
//! This crate's aim is to bring a one-stop solution to the problem of compiling/linking C libraries
//! (and therefore *-sys crates) by using the least amount of friction possible.
//!
//! # Practical Setup
//!
//! In your *-sys crate's build script, which will be using `bindgen` and `cc`, you can access the
//! includes folder that `c-compat` exposes via the `DEP_C_COMPAT_METADATA` environmental variable.
//!
//!
//! ```no_run
//! let c_compat_includes = std::env::var("DEP_C_COMPAT_METADATA").expect("DEP_C_COMPAT_METADATA not set");
//! let c_compat_includes = c_compat_includes.strip_prefix("include=").unwrap();
//! ```
//!
//! You can then make sure that bindgen discards any system includes (which vary across versions,
//! might not be available, or simply risk to not be architecture independent) and use the
//! `c-compat` includes as a substitute:
//!
//! ```no_run
//!     let bindings = bindgen::Builder::default()
//!         .clang_arg(&format!("-I{c_compat_includes}"))
//!         .clang_arg("-nostdinc")
//!         .clang_arg("-nobuiltininc")
//!         ...
//! ```
//!
//! For `cc` just make sure the headers are available and we also are not using any standard
//! included headers:
//!
//! ```no_run
//!     let mut cc_build = cc::Build::new();
//!     cc_build
//!         .include(&c_compat_includes)
//!         .flag("-nostdinc")
//!         ...
//! ```
//!
//! Then in your *-sys crate you can import the symbols so that they are visible by the linker
//!
//! ```no_run
//! // Make sure we automatically make the necessary symbols available
//! use c_compat as _;
//! ```
//!
//! and that's it! As long as all the `c-compat` includes cover the needs of your *-sys crate, you
//! should be in a position where you are leveraging the power of `clang` to cross-compile a C
//! library with Rust without needing any extra sysroots, compilers, etc...
#![no_std]

pub mod ctype;
pub mod math;
pub mod stdlib;
pub mod string;
