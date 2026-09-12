#![allow(non_snake_case)]
#![allow(internal_features)]
#![allow(nonstandard_style)]
#![feature(repr_simd)]

pub mod M;
pub mod mem;
pub mod simd;

pub mod pre {
    pub use crate::{C, F, I, mem::memmove, simd::xmm_t};
}

pub type I = i32;
pub type F = f64;
pub type C = char;
