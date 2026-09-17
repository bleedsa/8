use std::rc::Rc;
use crate::{M::MTy, asm::ExePage};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FunPtr<'m> {
    pub name: Option<&'static str>,
    pub args: &'m [MTy],
    pub ret: MTy,
}

#[derive(Clone)]
pub struct VM<'m> {
    pub pages: Vec<Rc<ExePage>>,
    /** array of compiled functions */
    pub funs: Vec<FunPtr<'m>>,
}

impl<'m> VM<'m> {
    pub fn new() -> Self {
        Self {
            pages: Vec::new(),
            funs: Vec::new(),
        }
    }

    pub fn add_page(&'m mut self, exe: ExePage) -> Rc<ExePage> {
        self.pages.push(exe.into());
        let r = (&self.pages[self.pages.len() - 1]).clone();
        r
    }
}

#[macro_export]
macro_rules! mkasmfuns {
    ($vm:expr => [
        $(fn $i:ident($($t:ty),*) -> $r:ty => $s:expr;)*
    ]) => {{
        $crate::mkasmfuns!($vm => [
            $(
                fn $i[Asm::new()]($($t),*) -> $r
                => $s;
            )*
        ])
    }};

    ($vm:expr => [
        $(
            fn $i:ident[$a:expr]($($t:ty),*$(,)*)->$r:ty
            => $x:expr;
        )*
    ]) => {{
        use $crate::{vm::VM, asm::{ExePage, Asm}};

        $(fn $i<'m>(vm: &'m mut VM<'m>) -> R<&'m extern "C" fn($($t),*) -> $r> {
            /* stringify the fn name */
            let n = stringify!($i);

            /* make the assembler */
            let asm: Asm = $a.emit_fun::<${count($t)}, _, _>(n, $x)?;

            /* get the exec page */
            let exe: Rc<ExePage> = vm.add_page(asm.exe()?);

            /* setup the page */
            let fun: &'m _ = unsafe { fun!(exe, fn($($t),*) -> $r = n) };
            Ok(fun)
        })*

        ($($i(&mut $vm)?),*)
    }};
}

#[cfg(test)]
#[cfg(not(miri))]
mod nomiri {
    use super::*;
    use crate::pre::*;

    #[test]
    fn basic_asm_fun() -> R<()> {
        let mut vm = VM::new();

        let add = mkasmfuns!(vm => [
            fn add(I, I) -> I =>
                "
                mov eax, edi
                add eax, esi
                ret
                ";
        ]);

        assert_eq!(add(1, 2), 3);

        Ok(())
    }

    #[test]
    fn eval_add_F() -> R<()> {
        let mut vm = VM::new();

        let add = mkasmfuns!(vm => [
            fn add(F, F) -> F =>
                "
                addsd xmm0, xmm1
                movd rax, xmm0
                ret
                ";
        ]);

        assert_eq!(add(1.0, 2.0), 3.0);

        Ok(())
    }

    /*
    #[test]
    fn eval_add_M() -> R<()> {
        let mut vm = VM::new();

        let add_m = mkasmfuns!(vm => [
            fn add(M, M) -> I
            => asm, (x=>x), (y=>y)
            => {
                asm
                    .define_const("XT", x.ty as u8)
                    .define_const("YT", y.ty as u8)
                    .define_const("Xi", To::<I>::to(x))
                    .define_const("Yi", To::<I>::to(y))

                Ok(())
            };
        ]);

        assert_eq!(3, add_m(1.to(), 2.to()));

        Ok(())
    }
    */
}
