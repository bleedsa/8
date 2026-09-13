#[inline(always)]
pub unsafe fn memcpy<X, Y>(x: *mut X, y: *const Y, z: usize) {
    unsafe {
        xxx::cpy(x as *mut u8, y as *mut u8, z);
    }
}
