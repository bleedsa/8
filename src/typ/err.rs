use std::{error::Error, fmt};
use crate::typ::name::Name;

/** type err repr */
pub enum TypErr {
}

impl Error for TypErr {}

impl fmt::Debug for TypErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TypErr::*;

        write!(f, "'inference: ")?;
        match self {
            _ => todo!()
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
