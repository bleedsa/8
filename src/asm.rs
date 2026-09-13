use asm_rs::{Arch, Assembler, AssemblyResult};
use libc::{
    MAP_ANONYMOUS, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE, mmap, size_t,
};

use std::{ptr::{self, NonNull}};
use crate::{pre::*, asm::err::pre::*, intern, M::M};

pub mod err;

#[cfg(test)]
#[cfg(not(miri))]
pub mod nomiri;

pub unsafe fn mmap_exec(z: usize) -> NonNull<u8> {
    let p_flags: i32 = (PROT_READ | PROT_WRITE | PROT_EXEC) as i32;
    let m_flags: i32 = (MAP_PRIVATE | MAP_ANONYMOUS) as i32;

    unsafe {
        let r = mmap(ptr::null_mut(), z as size_t, p_flags, m_flags, -1, 0) as *mut u8;

        if let Some(p) = NonNull::new(r) {
            p
        } else {
            panic!("failed to mmap() {z} bytes.");
        }
    }
}

/** NOTE: miri does not support calls to mmap with PROT_EXEC */
#[cfg(not(miri))]
#[test]
fn basic_mmap() {
    unsafe {
        let map = mmap_exec(4096);

        for i in 0..4096 {
            *map.as_ptr().add(i) = i as u8;
            assert_eq!(*map.as_ptr().add(i), i as u8);
        }
    }
}

pub struct Fun {
    pub name: &'static str,
    pub args: usize,
}

impl Fun {
    #[inline(always)]
    pub fn new<const A: usize, N>(n: N) -> Self
    where
        N: ToString,
    {
        Self {
            name: intern::str::add(n.to_string()),
            args: A,
        }
    }
}

pub struct Asm {
    pub asm: Assembler,
    pub funs: Vec<Fun>,
}

impl Asm {
    pub fn new() -> Self {
        Self {
            asm: Assembler::new(Arch::X86_64),
            funs: Vec::new(),
        }
    }

    pub fn emit<S>(&mut self, s: S) -> R<()>
    where
        S: AsRef<str>,
    {
        let _ = unS!(self.asm.emit(s.as_ref()));
        Ok(())
    }

    pub fn emit_fun<const A: usize, N, S>(&mut self, n: N, s: S) -> R<()>
    where
        N: AsRef<str>,
        S: AsRef<str>,
    {
        /* into */
        let n = n.as_ref();
        let s = s.as_ref();

        /* add to function stack */
        self.funs.push(Fun::new::<A, _>(n));

        /* emit */
        unS!(self.asm.label(n));
        unS!(self.asm.emit(s));

        Ok(())
    }

    pub fn exe(self) -> R<Exe> {
        /* grab the assembler results */
        let res = unS!(self.asm.finish());
        let bs = res.bytes();               /* get the bytes */
        let bL = bs.len();                  /* number of bytes */
        let map = unsafe { mmap_exec(bL) }; /* make an executable mem page */

        /* copy the asm into the page */
        unsafe {
            memcpy(map.as_ptr(), bs.as_ptr(), bL);
        }

        Ok(Exe {
            map,
            res,
        })
    }
}

pub struct Exe{
    pub map: NonNull<u8>,
    pub res: AssemblyResult,
}

#[macro_export]
macro_rules! fun {
    ($exe:expr, fn($($a:ty),*$(,)*) -> $r:ty = $n:expr) => {{
        use std::mem::transmute;

        /* get label address offset */
        let off = $exe.label($n)?;
        /* get function pointer based on offset */
        let ptr = $exe.map.as_ptr().add(off);
        /* transmute the pointer into a function */
        let fun: extern "C" fn($($a),*) -> $r = transmute(ptr);

        fun
    }};
}

impl Exe {
    #[inline(always)]
    pub fn label<N>(&self, n: N) -> R<usize>
    where
        N: AsRef<str>,
    {
        let n = n.as_ref();
        Ok(self.res
            .label_address(n)
            .ok_or(AsmErr::LabelNotFound(n.to_string()))? as usize)
    }

    #[inline(always)]
    pub unsafe fn fun0<N, T>(&self, n: N) -> R<extern "C" fn() -> T> 
    where
        N: AsRef<str>
    {
        unsafe {
            Ok(fun!(self, fn() -> T = n.as_ref()))
        }
    }
}
