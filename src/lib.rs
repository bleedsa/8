#![allow(non_snake_case)]
#![allow(internal_features)]
#![allow(nonstandard_style)]
#![feature(repr_simd)]
#![feature(likely_unlikely)]
#![feature(core_intrinsics)]

use crate::{M::val_t, pre::*};
use std::{
    error::Error, fmt, intrinsics::simd::simd_splat, mem::MaybeUninit as U,
};

pub mod M;
pub mod asm;
pub mod intern;
pub mod mem;
pub mod simd;
pub mod vm;

pub mod pre {
    pub use crate::{
        C, E, F, I,
        M::err::MErr,
        Pos, R, To, fatal,
        mem::memcpy,
        simd::{xmm_t, ymm_t},
    };
}

pub type I = i32;
pub type F = f64;
pub type C = char;

pub type R<T> = Result<T, Box<dyn Error>>;

/** wrap an R error */
#[macro_export]
macro_rules! E {
    ($e:expr) => {{ Err(Box::new($e)) }};
}

#[macro_export]
macro_rules! fatal {
    ($($x:tt)*) => {{
        eprintln!("FATAL ERROR: {}", format!($($x)*));
        std::process::exit(-1);
    }};
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(packed)]
pub struct Pos(pub u32, pub u32, pub u32);

impl Pos {
    #[inline(always)]
    pub fn line(&self) -> usize {
        self.0 as usize
    }

    #[inline(always)]
    pub fn col(&self) -> usize {
        self.1 as usize
    }
}

impl fmt::Display for Pos {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.line(), self.col())
    }
}

impl Default for Pos {
    #[inline(always)]
    fn default() -> Self {
        Self(0, 0, 0)
    }
}

#[unsafe(link_section = ".ctor")]
pub static CTOR_INIT: extern "C" fn() = c_init;

extern "C" fn c_init() {
    intern::init();
}

pub fn init() {
    c_init();
}

pub trait To<X> {
    fn to(self) -> X;
}

macro_rules! _impl_to {
    [$I:ty => $($T:ty),* $(,)*] => {
        $(
            impl To<$I> for $T {
                #[inline(always)]
                fn to(self) -> $I {
                    debug_assert!(size_of::<$I>() >= size_of::<$T>());
                    unsafe {
                        let mut r: $I = simd_splat(0u8);
                        memcpy(&raw mut r, &raw const self, size_of::<$T>());
                        r
                    }
                }
            }

            impl To<$T> for $I {
                #[inline(always)]
                fn to(self) -> $T {
                    #[cfg(test)]
                    debug_assert!(size_of::<$T>() <= size_of::<$I>());
                    let mut r: U<$T> = U::uninit();
                    unsafe {
                        memcpy(&raw mut r, &raw const self, size_of::<$T>());
                        r.assume_init()
                    }
                }
            }
        )*
    };
}

_impl_to![val_t => I, F, C, *mut I, *mut F, *mut C];
