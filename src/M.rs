use crate::{intern::pos, pre::*};
use std::{
    cmp, fmt, intrinsics::simd::simd_splat, mem::MaybeUninit as U, rc::Rc,
    slice,
};

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

_impl_to![val_t => I, F, C, *mut I, *mut F, *mut C];

impl To<val_t> for Dyd {
    fn to(self) -> val_t {
        unsafe {
            let mut r: val_t = simd_splat(0u8);
            memcpy(
                &raw mut r,
                self.v.vec().as_ptr(),
                size_of::<[u8; VERB_LEN]>(),
            );
            r
        }
    }
}

impl To<Dyd> for val_t {
    fn to(self) -> Dyd {
        /* raw self */
        let mut ptr = &raw const self;

        unsafe {
            /* copy verb name */
            const VZ: usize = size_of::<verb_t>();
            let mut v: U<verb_t> = U::uninit();
            let v = v.as_mut_ptr();
            memcpy(v.cast::<verb_t>(), ptr, VZ);
            ptr = ptr.add(VZ);
            let v = *v.cast::<verb_t>();

            /* arg size */
            const AZ: usize = size_of::<*const M>();

            /* x */
            let x: Rc<M> = Rc::from_raw(ptr as *const M);
            ptr = ptr.add(AZ);

            /* y */
            let y: Rc<M> = Rc::from_raw(ptr as *const M);

            let r = Dyd { v, x, y };

            r
        }
    }
}

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
            memcpy(
                a.as_mut_ptr(),
                s.bytes().collect::<Vec<_>>().as_ptr(),
                L as usize,
            );
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

type VerbArg<T> = Rc<T>;

/**
 * a dyadic verb. reference counted.
 *
 * `x` & `y` are alloc'd raw ptrs.
 * `v` is the verb str (max 4 chars).
 */
#[derive(Clone)]
pub struct Dyd {
    /** str, len */
    pub v: verb_t,
    pub x: VerbArg<M>,
    pub y: VerbArg<M>,
}

#[macro_export]
macro_rules! V {
    ($v:expr, $x:expr, $y:expr) => {{
        /* make the verb array */
        let v = verb_t::new(*$x.pos, $v)?;

        /* alloc x&y */
        let x = Rc::new($x.clone());
        let y = Rc::new($y.clone());

        Dyd { v, x, y }
    }};
}

impl Dyd {
    #[inline(always)]
    pub fn x<'a, X>(&'a self) -> X
    where
        &'a M: To<X>,
    {
        (&*self.x).to()
    }

    #[inline(always)]
    pub fn y<'a, X>(&'a self) -> X
    where
        &'a M: To<X>,
    {
        (&*self.y).to()
    }

    #[inline(always)]
    pub fn xty(&self) -> MTy {
        (&*self.x).ty
    }

    #[inline(always)]
    pub fn yty(&self) -> MTy {
        (&*self.y).ty
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
        /* v */
        write!(f, "({} ", self.v())?;

        macro_rules! fmt_arg {
            ($m:ident => $f:ident) => {{
                match self.$m() {
                    MTy::Int => write!(f, "{}", self.$f::<I>())?,
                    MTy::Dyd => write!(f, "{}", self.$f::<Dyd>())?,
                    t => fatal!("invalid MTy in Debug::fmt(): {t:?}"),
                }
            }};
        }

        fmt_arg!(xty=>x); /* x */
        write!(f, " ")?;
        fmt_arg!(yty=>y); /* y */

        write!(f, ")")
    }
}

impl fmt::Display for Dyd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PartialEq<Dyd> for Dyd {
    fn eq(&self, y: &Dyd) -> bool {
        let x = &*self;

        /* first make sure the verbs are the same */
        if x.v != y.v {
            return false;
        }

        self.x == y.x && self.y == y.y
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
    pub pos: &'static Pos,
    pub val: val_t,
}

impl M {
    pub fn pos(mut self, p: Pos) -> Self {
        let ptr = pos().add(p);
        self.pos = ptr;
        self
    }
}

impl Drop for M {
    fn drop(&mut self) {
        match self.ty {
            MTy::Dyd => {
                let d: Dyd = self.val.to();
            }
            _ => (),
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
                        pos: pos().add(Pos::default()),
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

impl To<M> for &M {
    fn to(self) -> M {
        self.clone()
    }
}

#[test]
fn M_val_valid_size() {
    println!("Zs: {} >= {}", size_of::<val_t>(), size_of::<Dyd>());
    assert!(size_of::<val_t>() >= size_of::<Dyd>());
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::init;

    #[test]
    fn mk_atoms() {
        init();

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
    fn mk_basic_oprs() -> R<()> {
        init();

        let x: M = 5i32.to();
        let y: M = 10i32.to();

        let o = V!("+", x, y);
        println!("{o:?}");
        assert!(o.v() == "+");
        assert!(5i32 == o.x());
        assert!(10i32 == o.y());

        let o = V!("=====", x, y);
        println!("{o:?}");
        assert!(o.v() == "====");
        assert!(5i32 == o.x());
        assert!(10i32 == o.y());

        Ok(())
    }

    #[test]
    fn mk_nest_dyds() -> R<()> {
        init();

        let x: M = 15i32.to();
        let y: M = 20i32.to();
        let o: M = V!("+", x, y).to();

        /* nest & check */
        let n = V!("@", o, y);
        println!("{n:?}");
        assert!(n.v() == "@");
        assert!(o == n.x());
        assert!(y == n.y());

        Ok(())
    }
}
