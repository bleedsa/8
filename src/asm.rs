use libc::{
    MAP_ANONYMOUS, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE, mmap, size_t,
};

use std::ptr;

pub unsafe fn mmap_exec(z: usize) -> *mut u8 {
    let p_flags: i32 = (PROT_READ | PROT_WRITE | PROT_EXEC) as i32;
    let m_flags: i32 = (MAP_PRIVATE | MAP_ANONYMOUS) as i32;

    unsafe {
        mmap(ptr::null_mut(), z as size_t, p_flags, m_flags, -1, 0) as *mut u8
    }
}

#[cfg(test)]
mod test {
    /** NOTE: miri does not support calls to mmap with PROT_EXEC */
    #[cfg(not(miri))]
    #[test]
    fn basic_mmap() {
        use super::mmap_exec;
        unsafe {
            let map = mmap_exec(4096);

            for i in 0..4096 {
                *map.add(i) = i as u8;
                assert_eq!(*map.add(i), i as u8);
            }
        }
    }
}
