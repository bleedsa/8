use crate::{M::MTy, intern, pre::*};
use std::sync::Mutex;

type TmpVal = u16;
static TMP_COUNT: Mutex<TmpVal> = Mutex::new(0);

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Typ {
    Var(Name),
    Arrow(&'static Typ, &'static Typ),
    Atom(MTy),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TypVar(pub &'static str);

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
pub struct TermVar(pub &'static str);

//#[derive(Clone, Debug, PartialEq)]
//pub struct Env(pub HashMap<TermVar, Scheme>);
