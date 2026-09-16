use crate::{M::M, enums::pre::*, pre::*, simd::Ixmm_t};
use std::{
    intrinsics::simd::{simd_add, simd_splat},
    mem::MaybeUninit,
    ptr,
    rc::Rc,
};

pub fn iota(n: usize) -> *mut I {
    let n_div_4 = n / 4;
    let ptr: *mut I = unsafe { xxx::new(n).expect("oom") };
    let xmm = ptr as *mut Ixmm_t;
    const BASE: Ixmm_t =
        unsafe { ptr::read_unaligned([0, 1, 2, 3].as_ptr() as *const Ixmm_t) };

    for i in 0..n_div_4 {
        unsafe {
            let v = simd_add(BASE, simd_splat((i * 4) as I));
            ptr::write_unaligned(xmm.add(i), v);
        }
    }

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
    let num = 27;
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
