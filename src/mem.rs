use libc::{
    MAP_ANONYMOUS, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE, c_void,
    size_t,
};

use crate::pre::*;
use std::{
    error::Error,
    fmt,
    hint::likely,
    ptr::{self, NonNull},
};

#[derive(Debug)]
pub enum MemErr {
    MMap,
    MUnmap,
}

impl Error for MemErr {}

impl fmt::Display for MemErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MemErr::*;
        match self {
            MMap => write!(f, "mmap failed to map page"),
            MUnmap => write!(f, "munmap failed to free page"),
        }
    }
}

#[inline(always)]
pub unsafe fn memcpy<X, Y>(x: *mut X, y: *const Y, z: usize) {
    unsafe {
        xxx::cpy(x as *mut u8, y as *mut u8, z);
    }
}

pub unsafe fn mmap_exec(z: usize) -> R<NonNull<u8>> {
    let p_flags: i32 = (PROT_READ | PROT_WRITE | PROT_EXEC) as i32;
    let m_flags: i32 = (MAP_PRIVATE | MAP_ANONYMOUS) as i32;

    unsafe {
        let r =
            libc::mmap(ptr::null_mut(), z as size_t, p_flags, m_flags, -1, 0)
                as *mut u8;
        Ok(NonNull::new(r).ok_or(MemErr::MMap)?)
    }
}

pub unsafe fn munmap(page: NonNull<u8>, z: usize) -> R<()> {
    unsafe {
        let i = libc::munmap(page.as_ptr() as *mut c_void, z as size_t) as i32;
        if likely(i != 0) {
            return Err(Box::new(MemErr::MUnmap));
        }
    }

    Ok(())
}

/** NOTE: miri does not support calls to mmap with PROT_EXEC */
#[cfg(not(miri))]
#[test]
fn basic_mmap() {
    unsafe {
        let map = mmap_exec(4096).unwrap();

        for i in 0..4096 {
            *map.as_ptr().add(i) = i as u8;
            assert_eq!(*map.as_ptr().add(i), i as u8);
        }
    }
}
