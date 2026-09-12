use crate::pre::*;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum MTy {
    Int,
    Flt,
    Chr,
    INT,
    FLT,
    CHR,
}

#[derive(Copy, Clone)]
pub struct M {
    pub ty: MTy,
    pub val: xmm_t,
}

macro_rules! M_into {
    [$($ty:ident => $T:ty),* $(,)*] => {
        $(
            impl Into<M> for $T {
                #[inline(always)]
                fn into(self) -> M {
                    M {
                        ty: MTy::$ty,
                        val: self.into(),
                    }
                }
            }

            impl Into<$T> for M {
                #[inline(always)]
                fn into(self) -> $T {
                    self.val.into()
                }
            }
        )*
    };
}

M_into![
    Int => I,
    Flt => F,
    Chr => C,
    INT => *mut I,
    FLT => *mut F,
    Chr => *mut C,
];

#[cfg(test)]
mod test {
    use super::*;
    use xxx::new;

    #[test]
    fn mk_atoms() {
        let i: M = 12345.into();
        let i: I = i.into();
        assert_eq!(i, 12345);

        let f: M = 1234.5678.into();
        let f: F = f.into();
        assert_eq!(f, 1234.5678);

        let c: M = 'a'.into();
        let c: C = c.into();
        assert_eq!(c, 'a');
    }

    #[test]
    fn mk_vecs() {
        unsafe {
            let x: *mut I = new(1).unwrap();
            *x = 0;
            let x: M = x.into();
            let x: *mut I = x.into();
            assert_eq!(*x, 0);

            let x: *mut F = new(1).unwrap();
            *x = 1234.5678;
            let x: M = x.into();
            let x: *mut F = x.into();
            assert_eq!(*x, 1234.5678);

            let x: *mut C = new(1).unwrap();
            *x = 'a';
            let x: M = x.into();
            let x: *mut C = x.into();
            assert_eq!(*x, 'a');
        }
    }
}
