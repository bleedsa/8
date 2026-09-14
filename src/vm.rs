use crate::{M::M, asm::ExePage, pre::*};

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
        $(fn $i:ident($($t:ty),*) -> $r:ty => $s:expr;)*
    ]) => {{
        /*
        $(fn $i<'m>(vm: &mut VM<'m>) -> R<extern "C" fn($($t),*) -> $r> {

        })
        */

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
            let exe: &'m ExePage = vm.add_page(asm.exe()?);

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
    fn eval_add_F() {
        let mut vm = VM::new();
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
