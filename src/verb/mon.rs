use crate::{
    A::A,
    M::{M, MTy},
    enums::pre::*,
    err_verb,
    pre::*,
    simd::{Ixmm_t, Iymm_t},
    verb::err::VerbErr,
};
use std::{
    intrinsics::simd::{simd_add, simd_splat},
    mem::MaybeUninit,
    ptr,
    rc::Rc,
    slice,
};

pub fn raw_iota(n: usize) -> *mut I {
    /* ptrs to write against */
    let ptr: *mut I = unsafe { xxx::new(n).expect("oom") };
    let mut ymm = ptr as *mut Iymm_t;

    /* base iota vec */
    const BASEY: *const Iymm_t =
        [0, 1, 2, 3, 4, 5, 6, 7].as_ptr() as *const Iymm_t;

    /* addition vecs */
    const AX: Ixmm_t = unsafe { simd_splat(4) };
    const AY: Iymm_t = unsafe { simd_splat(8) };

    /* ymm write */
    let mut vy = unsafe { ptr::read_unaligned(BASEY) };
    let mut rest = unsafe {
        for _ in 0..n / 8 {
            ptr::write_unaligned(ymm, vy);
            vy = simd_add(vy, AY);
            ymm = ymm.add(1);
        }

        ymm as *mut Ixmm_t
    };

    /* xmm write */
    let mut vx = unsafe { simd_add(AX, ptr::read_unaligned(rest.sub(1))) };
    let rest = unsafe {
        for _ in 0..n % 8 / 4 {
            ptr::write_unaligned(rest, vx);
            vx = simd_add(vx, AX);
            rest = rest.add(1);
        }

        rest
    };
    let rest = rest as *mut I;

    /* write remainder */
    unsafe {
        let v = ptr::read_unaligned(rest.sub(1)) as usize + 1;
        for i in 0..(n % 4) {
            ptr::write(rest.add(i), (i + v) as I);
        }
    }

    ptr
}

#[test]
fn mk_raw_iota_ptr() {
    let f = |num| {
        let ptr = raw_iota(num);

        for i in 0..num {
            unsafe {
                assert_eq!(*ptr.add(i), i as I);
            }
        }

        unsafe { xxx::free(ptr, num) };
    };

    f(127);
    f(1027);
    f(100);
    f(153);
    f(5239);
}

enum_jmps! {
    enum Mons
    derives(Copy, Clone, Debug, PartialEq)
    {
        Iota,
    }

    static EVAL_MONS: fn(Rc<M>) -> Result<Rc<M>, VerbErr> = [
        [Iota] = |x| match x.ty {
            MTy::Int => {
                let int: I = <M as Clone>::clone(&x).to();
                let num = int as usize;
                let raw = raw_iota(num);
                let slc = slice::from_raw_parts(raw, num);
                let ret: A<I> = slc.to();
                let m: M = ret.to();
                xxx::free(raw, num);
                Ok(m.into())
            },
            t => err_verb!(MonArgs(Mons::Iota, t)),
        },
    ];
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mon {
    v: Mons,
    x: Rc<M>,
    y: Rc<M>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn uneven_iota() {
        let iota = EVAL_MONS[Mons::Iota];
        let vec: A<I> = iota(127.to().into()).unwrap().a();

        for i in 0..127 {
            assert_eq!(vec[i], i as I);
        }
    }
}
