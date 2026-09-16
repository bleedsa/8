use crate::{fun::Fun, pre::*, tup::Tup, verb::pre::*, vec::A};
use std::{fmt::{self, Debug}, mem::ManuallyDrop as MD, rc::Rc};

pub mod err;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum MTy {
    Int,
    Flt,
    Chr,
    INT,
    FLT,
    CHR,
    Dyd,
    Mon,
    Fun,
    Tup,
}

impl fmt::Display for MTy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MTy::*;

        write!(
            f,
            "`{}",
            match self {
                Int => "i",
                Flt => "f",
                Chr => "c",
                INT => "I",
                FLT => "F",
                CHR => "C",
                Dyd => "v",
                Mon => "u",
                Fun => "o",
                Tup => "t",
           }
        )
    }
}

#[repr(C)]
pub union MVal {
    i: I,
    f: F,
    c: C,
    I: MD<A<I>>,
    F: MD<A<F>>,
    C: MD<A<C>>,
    v: MD<Rc<Dyd>>,
    u: MD<Rc<Mon>>,
    o: MD<Rc<Fun>>,
    t: MD<Rc<Tup>>,
}

pub struct M {
    pub ty: MTy,
    pub pos: Pos,
    pub val: MVal,
}

impl Clone for M {
    fn clone(&self) -> Self {
        use MTy::*;
        let ty = self.ty;
        let pos = self.pos;
        let val = &self.val;

        let val = unsafe {
            match ty {
                Int => MVal { i: val.i },
                Flt => MVal { f: val.f },
                Chr => MVal { c: val.c },
                INT => MVal { I: val.I.clone() },
                FLT => MVal { F: val.F.clone() },
                CHR => MVal { C: val.C.clone() },
                Dyd => MVal { v: val.v.clone() },
                Mon => MVal { u: val.u.clone() },
                Fun => MVal { o: val.o.clone() },
                Tup => MVal { t: val.t.clone() },
            }
        };

        M { ty, pos, val }
    }
}

impl Drop for M {
    fn drop(&mut self) {
        use MTy::*;

        unsafe {
            match self.ty {
                INT => MD::drop(&mut self.val.I),
                FLT => MD::drop(&mut self.val.F),
                CHR => MD::drop(&mut self.val.C),
                Dyd => MD::drop(&mut self.val.v),
                Mon => MD::drop(&mut self.val.u),
                Fun => MD::drop(&mut self.val.o),
                Tup => MD::drop(&mut self.val.t),
                _ => (),
            }
        }
    }
}

impl fmt::Debug for M {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /*
         * format atoms
         */
        macro_rules! atoms {
            [$($i:ident => $t:ty),* $(,)*] => {{
                match self.ty {
                    $(MTy::$i => write!(f, "{:?}", To::<$t>::to(self))),*,
                }
            }};
        }
        atoms![
            Int => I, Flt => F, Chr => C,
            INT => A<I>, FLT => A<F>, CHR => A<C>,
            Dyd => Rc<Dyd>, Mon => Rc<Mon>,
            Fun => Rc<Fun>, Tup => Rc<Tup>,
        ]
    }
}

impl PartialEq<M> for M {
    fn eq(&self, y: &M) -> bool {
        let x = &*self;

        /* match simple atoms */
        macro_rules! atoms {
            [$($x:ident => $X:ty),* $(,)*] => {{
                $(
                    if let (MTy::$x, MTy::$x) = (x.ty, y.ty) {
                        let x: $X = x.to();
                        let y: $X = y.to();
                        return x == y;
                    }
                )*
            }};
        }
        atoms![
            Int => I, Flt => F, Chr => C,
            INT => A<I>, FLT => A<F>, CHR => A<C>,
            Dyd => Rc<Dyd>, Mon => Rc<Mon>,
            Fun => Rc<Fun>, Tup => Rc<Tup>
        ];

        false
    }
}

macro_rules! M_to_impls_simple {
    [$($ty:ident => $r:ty => $p:ident),* $(,)*] => {
        $(
        impl To<$r> for M {
            fn to(self) -> $r {
                unsafe {
                    self.val.$p
                }
            }
        }

        impl To<$r> for &M {
            fn to(self) -> $r {
                unsafe {
                    self.val.$p
                }
            }
        }

        impl To<M> for $r {
            fn to(self) -> M {
                M {
                    ty: MTy::$ty,
                    pos: Pos::default(),
                    val: MVal {
                        $p: self,
                    }
                }
            }
        }
        )*
    };
}

M_to_impls_simple![
    Int => I => i,
    Flt => F => f,
    Chr => C => c,
];

macro_rules! M_to_impls_md {
    [$($ty:ident => $r:ty => $p:ident),* $(,)*] =>{
        $(
        impl To<$r> for M {
            fn to(self) -> $r {
                unsafe {
                    (*self.val.$p).clone()
                }
            }
        }

        impl To<$r> for &M {
            fn to(self) -> $r {
                unsafe {
                    (*self.val.$p).clone()
                }
            }
        }

        impl To<M> for $r {
            fn to(self) -> M {
                M {
                    ty: MTy::$ty,
                    pos: Pos::default(),
                    val: MVal {
                        $p: MD::<Self>::new(self),
                    }
                }
            }
        }
        )*
    };
}

M_to_impls_md![
    INT => A<I> => I,
    FLT => A<F> => F,
    CHR => A<C> => C,
    Dyd => Rc<Dyd> => v,
    Mon => Rc<Mon> => u,
    Fun => Rc<Fun> => o,
    Tup => Rc<Tup> => t,
];

macro_rules! M_to_impls_vecs {
    [$($n:ident => $t:ty => $v:ident),*$(,)*] => {
        $(
        impl To<M> for Vec<$t>
        where
            $t: Debug + PartialEq + Clone,
        {
            fn to(self) -> M {
                let x = To::<A<$t>>::to(self);
                To::<M>::to(x)
            }
        }

        impl To<Vec<$t>> for M
        where
            $t: Debug + PartialEq + Clone,
        {
            fn to(self) -> Vec<$t> {
                let x = To::<A<$t>>::to(self);
                To::<Vec<$t>>::to(x)
            }
        }
        )*
    };
}

M_to_impls_vecs![
    INT => I => I,
];

impl To<M> for &M {
    fn to(self) -> M {
        self.clone()
    }
}

impl To<M> for M {
    fn to(self) -> M {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mk_atoms() {
        let i: M = 12345.to();
        let i: I = i.to();
        assert_eq!(i, 12345);

        let f: M = 1234.5678.to();
        let f: F = f.to();
        assert_eq!(f, 1234.5678);

        let c: M = 'a'.to();
        let c: C = c.to();
        assert_eq!(c, 'a');
    }

    #[test]
    fn mk_vecs() {
        let x: A<I> = vec![1, 2, 3, 4, 5].to();
        let y: M = x.clone().to();
        let y: A<I> = y.to();
        assert_eq!(x, y);

        let x: A<F> = vec![1., 2., 3., 4., 5.,].to();
        let y: M = x.clone().to();
        let y: A<F> = y.to();
        assert_eq!(x, y);

        let x: A<C> = "abcdefABCDEF".chars().collect::<Vec<_>>().to();
        let y: M = x.clone().to();
        let y: A<C> = y.to();
        assert_eq!(x, y);
    }
}
