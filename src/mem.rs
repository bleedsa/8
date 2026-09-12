use libc::c_void;

#[inline(always)]
pub unsafe fn memmove<X, Y>(x: *mut X, y: *const Y, z: usize) {
    unsafe {
        libc::memmove(x as *mut c_void, y as *mut c_void, z);
    }
}
