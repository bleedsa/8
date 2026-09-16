use crate::pre::*;

#[derive(Copy, Clone)]
#[repr(simd)]
pub struct xmm_t([u8; 16]);

#[derive(Copy, Clone)]
#[repr(simd)]
pub struct ymm_t([u8; 32]);

#[derive(Copy, Clone)]
#[repr(simd)]
pub struct Ixmm_t([I; 4]);
