#![no_std]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;
#[cfg(not(feature = "std"))]
mod std {
    pub mod any {
        pub use core::any::*;
    }
    pub mod borrow {
        pub use alloc::borrow::*;
        pub use core::borrow::*;
    }
    pub mod convert {
        pub use core::convert::*;
    }
    pub mod cell {
        pub use core::cell::*;
    }
    pub mod cmp {
        pub use core::cmp::*;
    }
    pub mod error {
        pub use core::error::*;
    }
    pub mod ffi {
        pub use alloc::string::String as OsString;
        pub type OsStr = str;
    }
    pub mod fmt {
        pub use core::fmt::*;
    }
    pub mod hash {
        pub use core::hash::*;
    }
    pub mod iter {
        pub use core::iter::*;
    }
    pub mod mem {
        pub use core::mem::*;
    }
    pub mod ops {
        pub use core::ops::*;
    }
    pub mod panic {
        pub use core::panic::*;
    }
    pub mod str {
        pub use core::str::*;
    }
    pub mod string {
        pub use alloc::string::*;
    }
    pub mod result {
        pub use core::result::*;
    }
    pub mod slice {
        pub use core::slice::*;
    }
    pub mod sync {
        pub use alloc::sync::*;
        pub use core::sync::*;
    }
    pub mod vec {
        pub use alloc::vec;
        pub use alloc::vec::*;
    }
    pub use alloc::format;
}
#[cfg(not(feature = "std"))]
pub use crate::std::*;

#[cfg(feature = "std")]
pub use std::*;
