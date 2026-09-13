use crate::pre::*;
use std::{cmp, fmt, hint::unlikely, mem::MaybeUninit as U, ptr, slice, rc::Rc, borrow::Cow, intrinsics::simd::simd_splat};

pub mod err;

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
                        let mut r: $I = simd_splat(0u8);
                        memcpy(&raw mut r, &raw const self, size_of::<$T>());
                        r
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
                        memcpy(&raw mut r, &raw const self, size_of::<$T>());
                        r.assume_init()
                    }
                }
            }
        )*
    };
}

_impl_to![val_t => I, F, C, *mut I, *mut F, *mut C, Dyd];

pub const VERB_LEN: usize = 4;

/**
 * a verb string
 *
 * NOTE to skylar: does NOT include adverbs.
 * TODO: adverb_t
 */
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct verb_t(pub [u8; VERB_LEN], pub u32);

#[test]
fn verb_t_valid_size() {
    println!("Z: {}", size_of::<verb_t>());
    assert!(size_of::<verb_t>() <= 8);
}

pub static VERB_CHRS: &str = "!@#$%^&*_+-=~:<>?,|.";

impl verb_t {
    #[inline]
    pub fn new(p: Pos, s: &str) -> R<Self> {
        /* check if all chars are valid verb chars */
        if s.chars().any(|c| !VERB_CHRS.contains(c)) {
            return E!(MErr::InvalidVerb(p, s.to_string()));
        }

        /* make a buffer */
        let mut a = [0x43; VERB_LEN];

        /* get the length of the str <= 4 */
        let L = cmp::min(VERB_LEN, s.len());
        debug_assert!(L <= VERB_LEN);

        /* perform the copy */
        unsafe {
            memcpy(a.as_mut_ptr(), s.bytes().collect::<Vec<_>>().as_ptr(), L as usize);
        }

        Ok(Self(a, L as u32))
    }

    /** the actual char buffer */
    #[inline]
    pub fn vec<'a>(&'a self) -> &'a [u8; VERB_LEN] {
        &self.0
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.1 as usize
    }
}

type VerbArg<T> = Rc<Box<T>>;

/**
 * a dyadic verb. reference counted.
 *
 * `x` & `y` are alloc'd raw ptrs.
 * `v` is the verb str (max 4 chars).
 */
pub struct Dyd {
    /** str, len */
    pub v: verb_t,
    pub x: VerbArg<M>,
    pub y: VerbArg<M>,
}

#[macro_export]
macro_rules! V {
    ($v:expr, $x:expr, $y:expr) => {{
        unsafe {
            /* make the verb array */
            let v = verb_t::new((&$x).pos, $v)?;

            /* alloc x&y */
            let x = Rc::new(Box::new($x.clone()));
            let y = Rc::new(Box::new($y.clone()));

            Dyd { v, x, y }
        }
    }};
}

impl Dyd {
    #[inline(always)]
    pub fn x<'a, X>(&'a self) -> X
    where
        &'a M: To<X>,
    {
        unsafe { (&**self.x).to() }
    }

    #[inline(always)]
    pub fn y<'a, X>(&'a self) -> X
    where
        &'a M: To<X>,
    {
        unsafe { (&**self.y).to() }
    }

    #[inline(always)]
    pub fn xty(&self) -> MTy {
        unsafe { (&**self.x).ty }
    }

    #[inline(always)]
    pub fn yty(&self) -> MTy {
        unsafe { (&**self.y).ty }
    }

    #[inline]
    pub fn v(&self) -> &str {
        unsafe {
            let v = self.v.vec();
            let s = slice::from_raw_parts(v.as_ptr(), self.v_len());
            if let Ok(x) = str::from_utf8(s) {
                let _ = v;
                return x;
            } else {
                unreachable!()
            }
        }
    }

    #[inline(always)]
    pub fn v_len(&self) -> usize {
        self.v.len()
    }
}

impl fmt::Debug for Dyd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.yty(), self.xty()) {
            (MTy::Int, MTy::Int) => {
                let v = self.v();
                let x = self.x::<I>();
                let y = self.y::<I>();
                write!(f, "({v} {x:?}, {y:?})")
            }
            _ => todo!(),
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

pub type val_t = ymm_t;

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

#[derive(Clone)]
pub struct M {
    pub ty: MTy,
    pub pos: Pos,
    pub val: val_t,
}

impl M {
    pub fn pos(mut self, pos: Pos) -> Self {
        self.pos = pos;
        self
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
        atoms![Int => I, Flt => F, Chr => C];

        false
    }
}

macro_rules! M_impls {
    [$($ty:ident => $T:ty),* $(,)*] => {
        $(
            impl To<M> for $T {
                #[inline(always)]
                fn to(self) -> M {
                    M {
                        pos: Pos::default(),
                        ty: MTy::$ty,
                        val: self.to(),
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
    Dyd => Dyd,
];

#[test]
fn M_val_valid_size() {
    println!("Zs: {} >= {}", size_of::<val_t>(), size_of::<Dyd>());
    assert!(size_of::<val_t>() >= size_of::<Dyd>());
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
    fn mk_oprs() -> R<()> {
        let x: M = 5i32.to();
        let y: M = 10i32.to();

        let o = V!("+", x, y);
        println!("{o:?}");
        assert!(o.v() == "+");
        assert!(5i32 == o.x());
        assert!(10i32 == o.y());

        let o = V!("=====", x, y);
        println!("{o:?}");
        println!("{}", o.v());
        assert!(o.v() == "====");
        assert!(5i32 == o.x());
        assert!(10i32 == o.y());

        Ok(())
    }
}
