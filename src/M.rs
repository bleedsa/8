use crate::{
    pre::*,
    fun::Fun,
    tup::Tup,
};
use std::{cmp, fmt, mem::ManuallyDrop as MD, rc::Rc, slice};

pub mod err;

pub const VERB_LEN: usize = 4;

/**
 * a verb string
 *
 * NOTE to skylar: does NOT include adverbs.
 * TODO: adverb_t
 */
#[derive(Copy, Clone, Debug)]
pub struct verb_t(pub [u8; VERB_LEN], pub u32);

#[test]
fn verb_t_valid_size() {
    println!("Z: {}", size_of::<verb_t>());
    assert!(size_of::<verb_t>() <= 8);
}

#[test]
fn memcpy_verb_t() {
    use std::mem::MaybeUninit as U;
    let v = verb_t::new(Pos::default(), "+!@").unwrap();
    let mut r: U<verb_t> = U::uninit();
    unsafe {
        memcpy(r.as_mut_ptr(), &raw const v, size_of::<verb_t>());
        assert_eq!(v, r.assume_init());
    }
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
        let mut a = [0u8; VERB_LEN];

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

            Ok(Self(a, L as u32))
        }
    }

    #[inline]
    pub fn v(&self) -> &str {
        unsafe {
            let v = self.vec().as_ptr() as *const u8;
            let s = slice::from_raw_parts(v, self.len());
            if let Ok(x) = str::from_utf8(s) {
                let _ = v;
                return x;
            } else {
                unreachable!()
            }
        }
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

    #[inline(always)]
    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr().cast::<u8>()
    }
}

impl PartialEq for verb_t {
    fn eq(&self, y: &Self) -> bool {
        let L = self.len();

        /* check lens */
        if L != y.len() {
            return false;
        }

        let (x, y) = (self.as_ptr(), y.as_ptr());
        for i in 0..L {
            unsafe {
                if *x.add(i) != *y.add(i) {
                    return false;
                }
            }
        }

        true
    }
}

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
    pub a: Rc<(M, M)>,
}

#[macro_export]
macro_rules! V {
    ($v:expr, $x:expr, $y:expr) => {{
        use std::rc::Rc;
        use $crate::{
            M::{Dyd, M, verb_t},
            To,
        };

        let x = To::<M>::to($x);
        let y = To::<M>::to($y);

        /* make the verb array */
        let v = verb_t::new(x.pos, $v)?;

        /* alloc x&y */
        let a = Rc::new((x, y));

        Rc::new(Dyd { v, a })
    }};
}

impl Dyd {
    #[inline(always)]
    pub fn x<'a, X>(&'a self) -> X
    where
        &'a M: To<X>,
    {
        (&(*self.a).0).to()
    }

    #[inline(always)]
    pub fn y<'a, X>(&'a self) -> X
    where
        &'a M: To<X>,
    {
        (&(*self.a).1).to()
    }

    #[inline(always)]
    pub fn xty(&self) -> MTy {
        (&(*self.a).0).ty
    }

    #[inline(always)]
    pub fn yty(&self) -> MTy {
        (&(*self.a).1).ty
    }

    #[inline(always)]
    pub fn v(&self) -> &str {
        self.v.v()
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
                    MTy::Flt => write!(f, "{}", self.$f::<F>())?,
                    MTy::Dyd => write!(f, "{}", self.$f::<Rc<Dyd>>())?,
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

        self.a.0 == y.a.0 && self.a.1 == y.a.1
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum MTy {
    Int,
    Flt,
    Chr,
    Dyd,
    Fun,
    Tup,
}

impl fmt::Display for MTy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MTy::*;

        write!(f, "`{}", match self {
            Int => "i",
            Flt => "f",
            Chr => "c",
            Dyd => "v",
            Fun => "o",
            Tup => "t",
        })
    }
}

#[repr(C)]
pub union MVal {
    i: I,
    f: F,
    c: C,
    v: MD<Rc<Dyd>>,
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
                Dyd => MVal { v: val.v.clone() },
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
                Dyd => MD::drop(&mut self.val.v),
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
            Dyd => Rc<Dyd>, Fun => Rc<Fun>, Tup => Rc<Tup>,
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
            Dyd => Rc<Dyd>, Fun => Rc<Fun>, Tup => Rc<Tup>
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
    Dyd => Rc<Dyd> => v,
    Fun => Rc<Fun> => o,
    Tup => Rc<Tup> => t,
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
    fn mk_basic_oprs() -> R<()> {
        let x: M = 5i32.to();
        let y: M = 10i32.to();

        let o = V!("+", &x, &y);
        println!("{o:?}");
        assert!(o.v() == "+");
        assert!(5i32 == o.x());
        assert!(10i32 == o.y());

        let o = V!("=====", &x, &y);
        println!("{o:?}");
        assert!(o.v() == "====");
        assert!(5i32 == o.x());
        assert!(10i32 == o.y());

        Ok(())
    }

    #[test]
    fn mk_nested_dyds() -> R<()> {
        let o: M = V!("+", 15i32, 20i32).to();

        /* nest & check */
        let n = V!("@", &o, 20i32);
        println!("{n:?}");
        assert!(n.v() == "@");
        assert!(o == n.x());
        assert!(20i32 == n.y());

        Ok(())
    }
}
