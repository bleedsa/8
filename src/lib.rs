#![allow(non_snake_case)]
#![allow(internal_features)]
#![allow(nonstandard_style)]
#![feature(repr_simd)]
#![feature(likely_unlikely)]

use std::error::Error;

pub mod M;
pub mod asm;
pub mod mem;
pub mod simd;
pub mod vm;

pub mod pre {
    pub use crate::{
        C, F, I, R, err_fmt,
        mem::memmove,
        simd::{xmm_t, ymm_t},
    };
}

pub type I = i32;
pub type F = f64;
pub type C = char;

pub type R<T> = Result<T, Box<dyn Error>>;

#[macro_export]
macro_rules! err_fmt {
    ($($t:tt)*) => {{
        Err(format!($($t)*))
    }};
}
