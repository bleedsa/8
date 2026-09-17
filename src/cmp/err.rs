use std::{fmt, error::Error};
use crate::M::M;

pub enum CmpErr {
    CantCmpTy(M),
}

impl Error for CmpErr {}

impl fmt::Debug for CmpErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CmpErr::*;
        match self {
            CantCmpTy(m) => write!(f, "cannot compile expression with type {}: {:?}", m.ty, m),
        }
    }
}

impl fmt::Display for CmpErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[macro_export]
macro_rules! err_cmp {
    ($e:ident($($x:expr),*$(,)*)) => {{
        Err(Box::new($crate::cmp::err::CmpErr::$e($($x),*)))
    }};
}
