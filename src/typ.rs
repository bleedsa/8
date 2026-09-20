/*!
 * type class type checker.
 *
 * [typechecker zoo](
 *  https://sdiehl.github.io/typechecker-zoo/type-classes/implementation.html
 * )
 */

use crate::{
    M::{M, MTy},
    err_typ, intern,
    pre::*,
    typ::err::TypErr,
    verb::{dyd::Dyds, mon::Mons},
};
use std::{collections::HashMap, rc::Rc, sync::LazyLock};

pub mod err;
pub mod name;

/** the repr for a type in the type system */
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Typ {
    /** a function arrow from x->y */
    Arrow(&'static Typ, &'static Typ),
    /** an atom type */
    Atom(MTy),
    /** a dynamic type */
    Dyn,
}

impl Typ {
    pub fn new() -> Self {
        Self::Atom(MTy::Nil)
    }
}

macro_rules! verb_sigs {
    ($e:ident: ($q:ty, $($t:ty),*$(,)*)
     = [$($v:expr, $($a:expr),* => $r:expr),*$(,)*]
    ) => {
        #[allow(unused)]
        use MTy::*;
        #[allow(unused)]
        use Typ::*;
        #[allow(unused)]
        use Mons::*;
        #[allow(unused)]
        use Dyds::*;
        static $e: LazyLock<HashMap<($q, $(&'static $t),*), &Typ>> =
            LazyLock::new(|| {
                [$((($v, $(intern::typ::add($a)),*), intern::typ::add($r))),*].into()
            });
    };
}

verb_sigs!(DYD_SIGS: (Dyds, Typ, Typ) = [
    /* + */
    Add,Atom(Int),Atom(Int)=>Atom(Int),
    Add,Atom(Int),Atom(Flt)=>Atom(Flt),
    Add,Atom(Flt),Atom(Int)=>Atom(Flt),
    Add,Atom(Flt),Atom(Flt)=>Atom(Flt),
]);
verb_sigs!(MON_SIGS: (Mons, Typ) = [
    Iota,Atom(Int)=>Atom(INT),
]);

pub fn typ_of(m: Rc<M>) -> R<&'static Typ> {
    Ok(intern::typ::add(match m.ty {
        t @ (MTy::Int
        | MTy::Flt
        | MTy::Chr
        | MTy::INT
        | MTy::FLT
        | MTy::CHR) => Typ::Atom(t),

        MTy::Dyd => unsafe {
            /* snag */
            let v = m.val.v.v;
            let x = m.val.v.x.clone();
            let y = m.val.v.y.clone();

            /* project */
            if x.is_none() {
                todo!()
            }
            if y.is_none() {
                todo!()
            }

            /* typeof each */
            let x = typ_of(x.unwrap_unchecked())?;
            let y = typ_of(y.unwrap_unchecked())?;

            /* fetch overload return type */
            *(*DYD_SIGS)
                .get(&(v, x, y))
                .copied()
                .ok_or(Box::new(TypErr::DydNyi(v, *x, *y)))?
        },

        MTy::Mon => unsafe {
            /* snag */
            let v = m.val.u.v;
            let x = m.val.u.x.clone();

            /* project */
            if x.is_none() {
                todo!()
            }

            /* typeof arg */
            let x = typ_of(x.unwrap_unchecked())?;

            /* fetch overload return type */
            *(*MON_SIGS)
                .get(&(v, x))
                .copied()
                .ok_or(Box::new(TypErr::MonNyi(v, *x)))?
        },

        t => err_typ!(Nyi(t))?,
    }))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        M::{M, MTy},
        U, V,
        verb::dyd::Dyds,
        verb::mon::Mons,
    };

    #[test]
    fn typ_of_dyds() {
        let v: Rc<M> = V!(Dyds::Add, 123, 456).into();
        let t = typ_of(v).unwrap();
        assert_eq!(t, &Typ::Atom(MTy::Int));

        let x: Rc<M> = V!(Dyds::Add, 123, 456);
        let v: Rc<M> = V!(Dyds::Add, x, 789);
        let y = typ_of(v).unwrap();
        assert_eq!(t, &Typ::Atom(MTy::Int));
    }

    #[test]
    fn typ_of_mons() {
        let v: Rc<M> = U!(Mons::Iota, 10).into();
        let t = typ_of(v).unwrap();
        assert_eq!(t, &Typ::Atom(INT));
    }
}
