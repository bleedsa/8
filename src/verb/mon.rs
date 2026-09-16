use crate::{M::M, enums::pre::*, pre::*, simd::Ixmm_t};
use std::{
    intrinsics::simd::{simd_add, simd_splat},
    mem::MaybeUninit,
    ptr,
    rc::Rc,
};

pub fn iota(n: usize) -> *mut I {
    let n_div_4 = n / 4;

    /* ptrs to write against */
    let ptr: *mut I = unsafe { xxx::new(n).expect("oom") };
    let xmm = ptr as *mut Ixmm_t;
    
    /* simd vecs */
    let f: Ixmm_t = unsafe { simd_splat(4) };
    let v = [0, 1, 2, 3].as_ptr() as *const Ixmm_t;
    let mut v = unsafe { ptr::read_unaligned(v) };

    /* vectorize */
    for i in 0..n_div_4 {
        unsafe {
            ptr::write_unaligned(xmm.add(i), v);
            v = simd_add(v, f);
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
