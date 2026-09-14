use std::fmt;

pub mod pre {
    pub use crate::{asm::err::AsmErr, reS, unS};
}

pub enum AsmErr {
    Assembler(String),
    LabelNotFound(String),
}

impl std::error::Error for AsmErr {}

impl fmt::Debug for AsmErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AsmErr::*;
        match self {
            Assembler(e) => write!(f, "{e}"),
            LabelNotFound(l) => write!(f, "label not found in exec body: {l}"),
        }
    }
}

impl fmt::Display for AsmErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/** wrap an `asm-rs` error into an `AsmErr`. */
#[macro_export]
macro_rules! reS {
    ($x:expr) => {{ $x.map_err(|e| $crate::asm::err::AsmErr::Assembler(format!("{e}"))) }};
}

/** unwrap an `asm-rs` error into an `AsmErr` result. */
#[macro_export]
macro_rules! unS {
    ($x:expr) => {{ $crate::reS!($x)? }};
}
