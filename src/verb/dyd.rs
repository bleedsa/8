use crate::{
    M::{M, MTy},
    enums::pre::*,
    err_verb,
    pre::*,
    verb::err::VerbErr,
};
use std::{mem::MaybeUninit, rc::Rc};

macro_rules! math_dyd {
    ($o:tt) => {{
        |x: Rc<M>, y: Rc<M>| -> Result<Rc<M>, VerbErr> {
            use MTy::*;
            match (x.ty, y.ty) {
                (Int, Int) => {
                    let x: I = (&*x).to();
                    let y: I = (&*y).to();
                    let r: M = (x $o y).to();
                    Ok(r.into())
                }
                (Flt, Flt) => {
                    let x: F = (&*x).to();
                    let y: F = (&*y).to();
                    let r: M = (x $o y).to();
                    Ok(r.into())
                }
                (Int, Flt) => {
                    let x: I = (&*x).to();
                    let y: F = (&*y).to();
                    let r: M = (x as F $o y).to();
                    Ok(r.into())
                }
                (Flt, Int) => {
                    let x: F = (&*x).to();
                    let y: I = (&*y).to();
                    let r: M = (x $o y as F).to();
                    Ok(r.into())
                }
                (x, y) => err_verb!(DydArgs(Dyds::Add, x, y)),
            }
        }
    }};
}

enum_jmps! {
    enum Dyds
    derives(Copy, Clone, Debug, PartialEq)
    {
        Add,
        Sub,
        Mul,
        Div,
        App,
    }

    static EVAL_DYDS: fn(Rc<M>, Rc<M>) -> Result<Rc<M>, VerbErr> = [
        [Add] = math_dyd!(+),
        [Sub] = math_dyd!(-),
        [Mul] = math_dyd!(*),
        [Div] = math_dyd!(/),
        [App] = |_, _| todo!(),
    ];
}

#[derive(Clone, Debug, PartialEq)]
pub struct Dyd {
    v: Dyds,
    x: Rc<M>,
    y: Rc<M>,
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! math_tests {
        {$($n:ident = $s:ident[$i:expr] => [
            $(($x:expr, $y:expr, $z:expr, $T:ty)),*$(,)*
        ];)*} => {
            $(
            #[test]
            fn $n() {
                let f = $s[$i];

                $({
                    let x: Rc<M> = $x.to().into();
                    let y: Rc<M> = $y.to().into();
                    let r: $T = (&*f(x, y).unwrap()).clone().to();
                    assert_eq!(r, $z);
                })*
            }
            )*
        };
    }

    math_tests! {
        dyd_add = EVAL_DYDS[Dyds::Add] => [
            (1, 2, 3, I),
            (1.1, 1.2, 2.3, F),
            (1, 1., 2., F),
            (1., 1, 2., F)
        ];
        dyd_sub = EVAL_DYDS[Dyds::Sub] => [
            (5, 2, 3, I),
            (5.3, 0.3, 5.0, F),
            (1, 0.5, 0.5, F),
            (2.0, 1, 1.0, F),
        ];
        dyd_div = EVAL_DYDS[Dyds::Div] => [
            (10, 2, 5, I),
            (1., 2, 0.5, F),
            (9, 3., 3., F),
            (25., 5., 5., F),
        ];
    }
}
