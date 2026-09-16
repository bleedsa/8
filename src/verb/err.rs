use crate::{
    M::MTy,
    verb::{Dyds, Mons},
};
use std::{error::Error, fmt};

#[derive(Copy, Clone, PartialEq)]
pub enum VerbErr {
    DydArgs(Dyds, MTy, MTy),
    MonArgs(Mons, MTy),
}

impl Error for VerbErr {}

impl fmt::Debug for VerbErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use VerbErr::*;
        match self {
            DydArgs(v, x, y) => {
                write!(f, "invalid args passed to dyad. {v:?}[{x};{y}]")
            }
            MonArgs(v, x) => {
                write!(f, "invalid args passed to monad. {v:?}[{x}]")
            }
        }
    }
}

impl fmt::Display for VerbErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[macro_export]
macro_rules! err_verb {
    ($e:ident($($x:expr),*)) => {{
        use $crate::verb::err::VerbErr;
        Err(VerbErr::$e($($x),*))
    }};
}
