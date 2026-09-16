use crate::{
    M::{M, MTy},
    enums::pre::*,
    err_verb,
    pre::*,
    verb::err::VerbErr,
};
use std::{mem::MaybeUninit, rc::Rc};

pub mod pre {
    pub use super::{Dyd, Dyds, Mon, Mons};
}

pub mod err;

enum_jmps! {
    enum Dyds
    derives(Copy, Clone, Debug, PartialEq)
    {
        Add,
        App,
    }

    static EVAL_DYDS: fn(Rc<M>, Rc<M>) -> Result<Rc<M>, VerbErr> = [
        [Add] = |x, y| {
            use MTy::*;
            match (x.ty, y.ty) {
                (Int, Int) => {
                    let x: I = <M as Clone>::clone(&x).to();
                    let y: I = <M as Clone>::clone(&y).to();
                    let r: M = (x+y).to();
                    Ok(r.into())
                }
                (x, y) => err_verb!(DydArgs(Dyds::Add, x, y)),
            }
        },
        [App] = |_, _| todo!(),
    ];
}

enum_jmps! {
    enum Mons
    derives(Copy, Clone, Debug, PartialEq)
    {
        Iota,
    }

    static EVAL_MONS: fn(Rc<M>) -> Rc<M> = [
        [Iota] = |_| todo!()
    ];
}

#[derive(Clone, Debug, PartialEq)]
pub struct Dyd {
    v: Dyds,
    x: Rc<M>,
    y: Rc<M>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mon {
    v: Mons,
    x: Rc<M>,
    y: Rc<M>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_add() {
        let add = EVAL_DYDS[Dyds::Add];
        let x: Rc<M> = 1i32.to().into();
        let y: Rc<M> = 2i32.to().into();

        let r: M = (&*add(x, y).unwrap()).clone();
        let r: i32 = r.to();

        assert_eq!(r, 3i32);
    }
}
