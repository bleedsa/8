use crate::{intern, pre::*};
use std::sync::Mutex;

/** type variable names */

type TmpVal = u16;
static TMP_COUNT: Mutex<TmpVal> = Mutex::new(0);

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
#[repr(u8)]
pub enum Name {
    Tmp(TmpVal),
    Named(&'static str),
    Path(&'static [&'static str]),
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

    pub fn path<S>(s: Vec<S>) -> Self
    where
        S: ToString,
    {
        let v: Vec<&'static str> = s
            .into_iter()
            .map(|s| intern::str::add(s.to_string()).as_ref())
            .collect();
        Self::Path(intern::paths::add(v))
    }
}
