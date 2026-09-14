use crate::{asm::ExePage, pre::*, M::{M, MTy}};

pub struct Fun<'m> {
    /** reference to the start of the generated page */
    pub fun_ptr: &'m u8,
    /** function name */
    pub name: &'static str,
}

impl<'m> Fun<'m> {
    pub fn new(exe: &'m ExePage, name: &'static str) -> R<Self> {
        Ok(Self {
            fun_ptr: exe.fun_ptr(name)?,
            name,
        })
    }
}

pub struct VM<'m> {
    pub pages: Vec<ExePage>,
    /** array of compiled functions */
    pub funs: Vec<Fun<'m>>,
}

impl<'m> VM<'m> {
    pub fn new() -> Self {
        Self {
            pages: Vec::new(),
            funs: Vec::new(),
        }
    }

    pub fn add_page(&'m mut self, exe: ExePage) -> &'m ExePage {
        self.pages.push(exe);
        let r: &'m _ = &self.pages[self.pages.len() - 1];
        r
    }
}

#[macro_export]
macro_rules! mkasmfuns {
    ($vm:expr => [
        $(
            fn $i:ident($($t:ty),*$(,)*)->$r:ty=>$s:expr;
        )*
        $(,)*
    ]) => {{
        use $crate::{vm::VM, asm::{Asm, AsmFun}};
        $(fn $i<'m>(vm: &'m mut VM<'m>) -> R<&'m extern "C" fn($($t),*) -> $r> {
            /* stringify the fn name */
            let n = stringify!($i);

            /* make the assembler */
            let mut asm = Asm::new();
            asm.emit_fun::<${count($t)}, _, _>(n, $s)?;

            /* get the exec page */
            let exe = vm.add_page(asm.exe()?);

            /* setup the page */
            let fun = unsafe { fun!(exe, fn($($t),*) -> $r = n) };

            Ok(fun)
        })*

        ($($i(&mut $vm)?),*)
    }};
}           

#[cfg(test)]
#[cfg(not(miri))]
mod nomiri {
    use super::*;

    #[test]
    fn basic_asm_fun() -> R<()> {
        let mut vm = VM::new();

        let (add) = mkasmfuns!(vm => [
            fn add(I, I) -> I =>
                "
                mov eax, rdi
                add eax, rsi
                ret
                ";

        ]);

        assert_eq!(add(1, 2), 3);

        Ok(())
    }
}
