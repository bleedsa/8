use asm_rs::{Arch, Assembler, AssemblyResult};

use crate::{
    M::M,
    asm::err::pre::*,
    intern,
    mem::{mmap_exec, munmap},
    pre::*,
};
use std::ptr::NonNull;

pub mod err;

#[cfg(test)]
#[cfg(not(miri))]
pub mod nomiri;

pub struct AsmFun {
    pub page: ExePage,
    pub name: &'static str,
    pub args: usize,
}

impl AsmFun {
    #[inline(always)]
    pub fn new<const A: usize, N>(n: N, page: ExePage) -> Self
    where
        N: ToString,
    {
        Self {
            page,
            name: intern::str::add(n.to_string()),
            args: A,
        }
    }
}

pub trait AsmDefine {
    fn define(self, asm: &mut Asm) -> R<&mut Asm>;
}

impl AsmDefine for u8 {
    fn define(self, asm: &mut Asm) -> R<&mut Asm> {
        let _ = unS!(asm.asm.db(&[self]));
        Ok(asm)
    }
}

impl AsmDefine for I {
    fn define(self, asm: &mut Asm) -> R<&mut Asm> {
        let _ = unS!(asm.asm.dd(self as u32));
        Ok(asm)
    }
}

impl AsmDefine for F {
    fn define(self, asm: &mut Asm) -> R<&mut Asm> {
        let _ = unS!(asm.asm.dq(self as u64));
        Ok(asm)
    }
}

pub struct Asm {
    pub asm: Assembler,
}

impl Asm {
    pub fn new() -> Self {
        Self {
            asm: Assembler::new(Arch::X86_64),
        }
    }

    pub fn emit<S>(&mut self, s: S) -> R<()>
    where
        S: AsRef<str>,
    {
        let _ = unS!(self.asm.emit(s.as_ref()));
        Ok(())
    }

    pub fn emit_fun<const A: usize, N, S>(mut self, n: N, s: S) -> R<Self>
    where
        N: AsRef<str>,
        S: AsRef<str>,
    {
        /* into */
        let n = n.as_ref();
        let s = s.as_ref();

        /* emit */
        unS!(self.asm.label(n));
        unS!(self.asm.emit(s));

        Ok(self)
    }

    pub fn define_const<N, X>(&mut self, n: N, x: X) -> &mut Self
    where
        N: ToString,
        X: AsmDefine,
    {
        let n = intern::str::add(n.to_string());
        let _ = x.define(self);
        self
    }

    pub fn exe(self) -> R<ExePage> {
        /* grab the assembler results */
        let res = unS!(self.asm.finish());
        let bs = res.bytes(); /* get the bytes */
        let bL = bs.len(); /* number of bytes */
        let map = unsafe { mmap_exec(bL)? }; /* make an executable mem page */

        /* copy the asm into the page */
        unsafe {
            memcpy(map.as_ptr(), bs.as_ptr(), bL);
        }

        Ok(ExePage::new(map, res))
    }
}

pub struct ExePage {
    pub map: NonNull<u8>,
    pub res: AssemblyResult,
}

impl AsRef<u8> for ExePage {
    fn as_ref<'m>(&'m self) -> &'m u8 {
        unsafe { &*self.map.as_ptr() }
    }
}

impl Drop for ExePage {
    fn drop(&mut self) {
        unsafe {
            if let Err(e) = munmap(self.map, self.res.bytes().len()) {
                fatal!("ExePage::drop(): {e}")
            }
        }
    }
}

impl ExePage {
    pub fn new(map: NonNull<u8>, res: AssemblyResult) -> Self {
        Self { map, res }
    }

    #[inline(always)]
    pub fn label<N>(&self, n: N) -> R<usize>
    where
        N: AsRef<str>,
    {
        let n = n.as_ref();
        Ok(self
            .res
            .label_address(n)
            .ok_or(AsmErr::LabelNotFound(n.to_string()))? as usize)
    }

    #[inline(always)]
    pub fn fun_ptr<'m, N>(&'m self, n: N) -> R<&'m u8>
    where
        N: AsRef<str>,
    {
        let off = self.label(n)?;
        Ok(unsafe { &*self.map.as_ptr().add(off) })
    }

    #[inline(always)]
    pub unsafe fn fun0<'m, N, T>(&self, n: N) -> R<&'m extern "C" fn() -> T>
    where
        N: AsRef<str>,
    {
        unsafe { Ok(fun!(self, fn() -> T = n.as_ref())) }
    }
}

#[macro_export]
macro_rules! fun {
    ($exe:expr, fn($($a:ty),*$(,)*) -> $r:ty = $n:expr) => {{
        use std::mem::transmute;

        /* get label address offset */
        let off = $exe.label($n)?;
        /* get function pointer based on offset */
        let ptr = &$exe.map.as_ptr().add(off);
        /* transmute the pointer into a function */
        let fun: &extern "C" fn($($a),*) -> $r = transmute(ptr);

        fun
    }};
}
