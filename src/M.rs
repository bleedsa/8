use crate::pre::*;
use std::{mem::MaybeUninit as U, hint::unlikely, fmt, ptr};

pub trait To<X> {
    fn to(self) -> X;
}

macro_rules! _impl_to {
    [$I:ty => $($T:ty),* $(,)*] => {
        $(
            impl To<$I> for $T {
                #[inline(always)]
                fn to(self) -> $I {
                    debug_assert!(size_of::<$I>() >= size_of::<$T>());
                    unsafe {
                        let mut r: U<$I> = U::uninit();
                        memmove(&raw mut r, &raw const self, size_of::<$T>());
                        r.assume_init()
                    }
                }
            }

            impl To<$T> for $I {
                #[inline(always)]
                fn to(self) -> $T {
                    #[cfg(test)]
                    debug_assert!(size_of::<$T>() <= size_of::<$I>());
                    let mut r: U<$T> = U::uninit();
                    unsafe {
                        memmove(&raw mut r, &raw const self, size_of::<$T>());
                        r.assume_init()
                    }
                }
            }
        )*
    };
}

_impl_to![ymm_t => I, F, C, *mut I, *mut F, *mut C, *mut Dyd];
_impl_to![xmm_t => I, F, C, *mut I, *mut F, *mut C, *mut Dyd];
_impl_to![u64   => I, F, C, *mut I, *mut F, *mut C, *mut Dyd];

pub struct Dyd {
    pub v: [char; 4],
    pub rc: u16,
    pub x: *mut M,
    pub y: *mut M,
}

#[macro_export]
macro_rules! V {
    ($v:expr, $x:expr, $y:expr) => {{
        use std::cmp;
        unsafe {
            /* make the verb array */
            let mut v = ['\0'; 4];
            let vL = cmp::min(4, $v.len());
            memmove(&raw mut v, $v.as_ptr(), vL);

            /* alloc x&y */
            let x = xxx::new(1)?; *x = $x;
            let y = xxx::new(1)?; *y = $y;

            Dyd {
                v,
                rc: 1,
                x,
                y,
            }
        }
    }};
}

impl Dyd {
    #[inline(always)]
    pub fn x<X>(&self) -> X
    where
        M: To<X>,
    {
        unsafe { To::<X>::to(*self.x) }
    }

    #[inline(always)]
    pub fn y<X>(&self) -> X
    where
        M: To<X>,
    {
        unsafe { To::<X>::to(*self.y) }
    }
}

impl fmt::Debug for Dyd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Dyd { v, rc: _, x, y } = self;
        unsafe {
            match ((**x).ty, (**y).ty) {
                (MTy::Int, MTy::Int) => write!(f, "({:?} {:?}, {:?})", v, self.x::<I>(), self.y::<I>()),
                _ => todo!()
            }
        }
    }
}

impl PartialEq<Dyd> for Dyd {
    fn eq(&self, y: &Dyd) -> bool {
        let x = &*self;

        /* first make sure the verbs are the same */
        if x.v != y.v {
            return false;
        }

        todo!()
    }
}

#[inline(always)]
unsafe fn free_dyad_arg(x: *mut M) {
    match unsafe { (*x).ty } {
        MTy::Dyd => unsafe {
            let x = x as *mut Dyd;
            let _ = ptr::read(x);
            xxx::free(x, 1);
        }
        _ => (),
    }
}

impl Drop for Dyd {
    fn drop(&mut self) {
        self.rc -= 1;
        if unlikely(self.rc <= 1) {
            unsafe {
                free_dyad_arg(self.x);
                free_dyad_arg(self.y);
            }
        }
    }
}

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
}

#[derive(Copy, Clone)]
pub struct M {
    pub ty: MTy,
    pub val: u64,
    pub rc: usize,
}

#[macro_export]
macro_rules! mty {
    (Int) => {I};
    (Flt) => {F};
    (Chr) => {C};
    (Dyd) => {*mut Dyd};
    ($t:ident) => {todo!()};
}

impl fmt::Debug for M {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* 
         * format atoms
         */
        macro_rules! atoms {
            [$($i:ident => $t:ty),* $(,)*] => {{
                match self.ty {
                    $(MTy::$i => return write!(f, "{}", To::<$t>::to(self.val))),*,
                    _ => (),
                }
            }};
        }
        atoms![Int => I, Flt => F, Chr => C];

        match self.ty {
            _ => unreachable!(),
        }
    }
}

impl PartialEq<M> for M {
    fn eq(&self, y: &M) -> bool {
        let x = &*self;

        macro_rules! mismatch_atoms {
            [$($x:ident, $y:ident => $X:ty, $Y:ty, $C:ty);* $(;)*] => {{
                match (x.ty, y.ty) {
                    $(
                        (MTy::$x, MTy::$y) => {
                            let x = To::<$X>::to(x) as $C;
                            let y = To::<$Y>::to(y) as $C;
                            return x == y;
                        }

                        (MTy::$y, MTy::$x) => {
                            let x = To::<$Y>::to(x) as $C;
                            let y = To::<$X>::to(y) as $C;
                            return x == y;
                        }
                    ),*,
                    _ => (),
                }
            }};
        }
        mismatch_atoms![
            Int, Flt => I, F, F;
            Chr, Int => C, I, u8;
        ];

        todo!()
    }
}

macro_rules! M_impls {
    [$($ty:ident => $T:ty),* $(,)*] => {
        $(
            impl To<M> for $T {
                #[inline(always)]
                fn to(self) -> M {
                    M {
                        ty: MTy::$ty,
                        val: self.to(),
                        rc: 1,
                    }
                }
            }

            impl To<$T> for M {
                #[inline(always)]
                fn to(self) -> $T {
                    self.val.to()
                }
            }

            impl To<$T> for &M {
                #[inline(always)]
                fn to(self) -> $T {
                    self.val.to()
                }
            }
        )*
    };
}

M_impls![
    Int => I,
    Flt => F,
    Chr => C,
    Dyd => *mut Dyd,
];

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
    fn mk_oprs() -> R<()> {
        let x: M = 5i32.to();
        let y: M = 10i32.to();
        let o = V!("+", x, y);

        assert_eq!(o.x::<I>(), 5);
        assert_eq!(o.y::<I>(), 10);

        Ok(())
    }
}
