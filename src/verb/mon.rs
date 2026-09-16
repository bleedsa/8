use crate::{M::M, enums::pre::*, pre::*, simd::{Iymm_t, Ixmm_t}};
use std::{
    intrinsics::simd::{simd_add, simd_splat},
    mem::MaybeUninit,
    ptr,
    rc::Rc,
};

pub fn iota(n: usize) -> *mut I {
    let n_div_8 = n / 8;
    let n_div_4 = n / 4;

    /* ptrs to write against */
    let ptr: *mut I = unsafe { xxx::new(n).expect("oom") };
    let ymm = ptr as *mut Iymm_t;
    let xmm = ptr as *mut Ixmm_t;

    /* ymm vecs */
    const BASEY: *const Iymm_t =
        [0, 1, 2, 3, 4, 5, 6, 7, 8].as_ptr() as *const Iymm_t;
    let fy: Iymm_t = unsafe { simd_splat(8) };
    let mut vy = unsafe { ptr::read_unaligned(BASEY) };

    const BASEX: *const Ixmm_t = BASEY as *const Ixmm_t;
    let fx: Ixmm_t = unsafe { simd_splat(4) };
    let mut vx = unsafe { ptr::read_unaligned(BASEX) };

    /* vectorize */
    for i in 0..n_div_8 {
        unsafe {
            ptr::write_unaligned(ymm.add(i), vy);
            vy = simd_add(vy, fy);
        }
    }

    unsafe {
        let last = n_div_8 * 2;
        let rest = xmm.add(last);
        for i in 0..(n % 8) / 4 {
            ptr::write_unaligned(rest.add(i), vx);
            vx = simd_add(vx, fx);
        }
    }

    /* write remainder */
    unsafe {
        let last = n_div_4 * 4;
        let rest = ptr.add(last);

        for i in 0..(n % 4) {
            ptr::write(rest.add(i), (i + last) as I);
        }
    }

    ptr
}

#[test]
fn raw_iota() {
    let num = 1027;
    let ptr = iota(num);

    for i in 0..num {
        unsafe {
            assert_eq!(*ptr.add(i), i as i32);
        }
    }

    unsafe { xxx::free(ptr, num) };
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
pub struct Mon {
    v: Mons,
    x: Rc<M>,
    y: Rc<M>,
}
