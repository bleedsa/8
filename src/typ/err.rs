use crate::{M::MTy, verb::{mon::Mons, dyd::Dyds}, typ::{Typ, name::Name}};
use std::{error::Error, fmt};

/** type err repr */
pub enum TypErr {
    Nyi(MTy),
    DydNyi(Dyds, Typ, Typ),
    MonNyi(Mons, Typ),
}

impl Error for TypErr {}

impl fmt::Debug for TypErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TypErr::*;

        write!(f, "'typ: ")?;
        match self {
            Nyi(t) => write!(f, "nyi: typechecking {t}"),
            DydNyi(v, x, y) => write!(f, "dyad nyi: {v:?}[{x:?};{y:?}]"),
            MonNyi(v, x) => write!(f, "monad nyi: {v:?}[{x:?}]"),
        }
    }
}

impl fmt::Display for TypErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/** create a type error */
#[macro_export]
macro_rules! err_typ {
    ($e:ident($($x:expr),*$(,)*)) => {{
        $crate::E!($crate::typ::err::TypErr::$e($($x),*))
    }};
}
