/*!
 * type checker.
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
        static $e: LazyLock<HashMap<($q, $($t),*), Typ>> =
            LazyLock::new(|| {
                #[allow(unused)]
                use MTy::*;
                #[allow(unused)]
                use Typ::*;
                #[allow(unused)]
                use Mons::*;
                #[allow(unused)]
                use Dyds::*;

                [
                    $((($v, $($a),*), $r)),*
                ].into()
            });
    };
}

/* dyadic verb type signatures */
verb_sigs!(DYD_SIGS: (Dyds, Typ, Typ) = [
    /* + */
    Add,Atom(Int),Atom(Int)=>Atom(Int),
    Add,Atom(Int),Atom(Flt)=>Atom(Flt),
    Add,Atom(Flt),Atom(Int)=>Atom(Flt),
    Add,Atom(Flt),Atom(Flt)=>Atom(Flt),
]);

/* monadic verb type signatures */
verb_sigs!(MON_SIGS: (Mons, Typ) = [
    Iota,Atom(Int)=>Atom(INT),
]);

#[derive(Default)]
pub struct TypChk {
    pub binds: HashMap<Name, Typ>,
}

impl TypChk {
    pub fn gets(&mut self, x: Rc<M>, y: Rc<M>) -> R<&'static Typ> {
        let x: &str = x.to().to();
        let t = self.typ_of(y)?;
        self.binds.insert(Name::Named(x), *t);
        Ok(t)
    }

    pub fn typ_of(&mut self, m: Rc<M>) -> R<&'static Typ> {
        Ok(intern::typ::add(match m.ty {
            t @ (MTy::Int
            | MTy::Flt
            | MTy::Chr
            | MTy::INT
            | MTy::FLT
            | MTy::CHR
            | MTy::Sym
            | MTy::SYM) => Typ::Atom(t),

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

                let x = x.unwrap_unchecked();
                let y = y.unwrap_unchecked();

                /* special cases (gets etc) */
                match v {
                    Dyds::Gets => return self.gets(x, y),
                    _ => ()
                };

                /* typeof each */
                let x = *self.typ_of(x)?;
                let y = *self.typ_of(y)?;

                /* fetch overload return type */
                let f = (*DYD_SIGS)
                    .get(&(v, x, y))
                    .copied()
                    .ok_or(Box::new(TypErr::DydNyi(v, x, y)))?;

                f
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
                let x = *self.typ_of(x.unwrap_unchecked())?;

                /* fetch overload return type */
                let f = (*MON_SIGS)
                    .get(&(v, x))
                    .copied()
                    .ok_or(Box::new(TypErr::MonNyi(v, x)))?;

                f
            },

            t => err_typ!(Nyi(t))?,
        }))
    }
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
        let mut H = TypChk::default();
        let v: Rc<M> = V!(Dyds::Add, Some(123), Some(456)).into();
        let t = H.typ_of(v).unwrap();
        assert_eq!(t, &Typ::Atom(MTy::Int));

        let x: Rc<M> = V!(Dyds::Add, Some(123), Some(456));
        let v: Rc<M> = V!(Dyds::Add, Some(x), Some(789));
        let t = H.typ_of(v).unwrap();
        assert_eq!(t, &Typ::Atom(MTy::Int));
    }

    #[test]
    fn typ_of_mons() {
        let mut H = TypChk::default();
        let v: Rc<M> = U!(Mons::Iota, Some(10)).into();
        let t = H.typ_of(v).unwrap();
        assert_eq!(t, &Typ::Atom(MTy::INT));
    }

    #[test]
    fn simple_gets() -> R<()> {
        let mut H = TypChk::default();
        let v: Rc<M> = V!(Dyds::Gets, Some("a"), Some(10)).into();
        let t = H.typ_of(v).unwrap();

        assert_eq!(t, &Typ::Atom(MTy::Int));

        Ok(())
    }
}
