#![allow(non_snake_case)]
#![allow(internal_features)]
#![allow(nonstandard_style)]
#![allow(static_mut_refs)]
#![feature(repr_simd)]
#![feature(likely_unlikely)]
#![feature(const_convert)]
#![feature(const_trait_impl)]
#![feature(macro_metavar_expr)]
#![feature(iter_intersperse)]
#![feature(core_intrinsics)]

use std::{error::Error, fmt};

pub mod M;
pub mod asm;
pub mod enums;
pub mod fun;
pub mod intern;
pub mod mem;
pub mod simd;
pub mod tup;
pub mod typ;
pub mod verb;
pub mod vm;
pub mod vec;

pub mod pre {
    pub use crate::{
        C, E, F, I,
        M::err::MErr,
        Pos, R, To, fatal, fun,
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

pub trait To<X> {
    fn to(self) -> X;
}
