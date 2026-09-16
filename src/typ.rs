use std::sync::Mutex;
use crate::{M::MTy, intern, pre::*};

type TmpVal = u16;
static TMP_COUNT: Mutex<TmpVal> = Mutex::new(0);

#[repr(u8)]
pub enum Name {
    Tmp(TmpVal),
    Named(&'static str),
}

impl Name {
    /** make a named variable */
    #[inline(always)]
    pub fn named<S>(s: S) -> Self
    where
        S: ToString,
    {
        Self::Named(intern::str::add(s.to_string()))
    }

    /** make a temporary variable */
    #[inline(always)]
    pub fn tmp() -> Self {
        let mut G = match TMP_COUNT.lock() {
            Ok(G) => G,
            Err(e) => fatal!("Name::tmp(): poisoned mutex: {e}"),
        };
        let v = *G;


        *G += 1;
        Self::Tmp(v)
    }
}

pub enum Typ {
    Var(Name),
    ETVar(&'static str),
    Arrow(&'static Typ, &'static Typ),
    Forall(Name, &'static Typ),
    Atom(MTy),
}
