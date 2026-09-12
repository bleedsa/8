use crate::pre::*;
use std::mem::{MaybeUninit as U, size_of};

#[derive(Copy, Clone)]
#[repr(simd)]
pub struct xmm_t([u8; 16]);

macro_rules! impl_xmm_into {
    [$($T:ty),* $(,)*] => {
        $(
            impl Into<xmm_t> for $T {
                #[inline(always)]
                fn into(self) -> xmm_t {
                    unsafe {
                        let mut r: U<xmm_t> = U::uninit();
                        memmove(&raw mut r, &raw const self, size_of::<$T>());
                        r.assume_init()
                    }
                }
            }

            impl Into<$T> for xmm_t {
                #[inline(always)]
                fn into(self) -> $T {
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

impl_xmm_into![I, F, C, *mut I, *mut F, *mut C,];
