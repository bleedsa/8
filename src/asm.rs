use libc::{
    MAP_ANONYMOUS, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE, mmap, size_t,
};

use crate::pre::*;
use std::{hint::likely, marker::PhantomData, ptr};

pub unsafe fn mmap_exec(z: usize) -> *mut u8 {
    let p_flags: i32 = (PROT_READ | PROT_WRITE | PROT_EXEC) as i32;
    let m_flags: i32 = (MAP_PRIVATE | MAP_ANONYMOUS) as i32;

    unsafe {
        mmap(ptr::null_mut(), z as size_t, p_flags, m_flags, -1, 0) as *mut u8
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Tape {
    pub ptr: *mut u8,
    pub z: usize,
    pub i: usize,
}

/** write bytes to a `Tape` */
#[macro_export]
macro_rules! putu8s {
    [$asm:expr => $($b:expr),* $(,)*] => {{
        let a = $asm;
        $(let a = unsafe { a.putb($b) };)*
        a
    }};
}

impl Tape {
    pub fn new() -> Self {
        let z = 1024;
        unsafe {
            Self {
                ptr: mmap_exec(4096),
                z,
                i: 0,
            }
        }
    }

    /** write a byte along the tape */
    #[inline(always)]
    pub unsafe fn putb(mut self, x: u8) -> Self {
        unsafe {
            if likely(self.i < self.z) {
                *self.ptr = x;
                self.ptr = self.ptr.add(1);
                self.i += 1;
            } else {
                todo!()
            }

            self
        }
    }

    pub fn bytes<'a>(&'a self) -> TapeBytes<'a> {
        TapeBytes {
            ptr: unsafe { self.ptr.sub(self.i) },
            i: 0,
            z: self.z,
            _mark: PhantomData,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TapeBytes<'a> {
    pub ptr: *mut u8,
    pub i: usize,
    pub z: usize,
    pub _mark: PhantomData<&'a ()>,
}

impl<'a> Iterator for TapeBytes<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if likely(self.i < self.z) {
            unsafe {
                let b = *self.ptr.add(self.i);
                self.i += 1;
                Some(b)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_mmap() {
        unsafe {
            let map = mmap_exec(4096);

            for i in 0..4096 {
                *map.add(i) = i as u8;
                assert_eq!(*map.add(i), i as u8);
            }
        }
    }

    #[test]
    fn putu8s() {
        let asm = putu8s![Tape::new() => 0, 1];
        let mut i = asm.bytes();
        assert_eq!(i.next(), Some(0x0));
        assert_eq!(i.next(), Some(0x1));
    }
}
