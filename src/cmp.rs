/*!
 * jit compilation
 */

use asm_rs::Arch;
use crate::{
    A::A,
    asm::Asm,
    fun::{Arg, Fun},
    pre::*,
    typ::{Typ, TypChk},
    vm::VM,
};
use std::rc::Rc;

pub mod err;

#[derive(PartialEq)]
pub struct CmpVar {
    typ: Typ,
}

#[derive(PartialEq)]
pub struct CmpFun {
    pub args: A<Arg>,
    pub ret: Typ,
    pub asm: Asm,
    pub tych: TypChk,
}

pub struct Cmp<'m> {
    pub vm: &'m mut VM<'m>,
    pub i: usize,
    pub tych: TypChk,
    pub arch: Arch,
}

impl<'m> Cmp<'m> {
    #[inline(always)]
    pub fn new(vm: &'m mut VM<'m>, arch: Arch) -> Self {
        Self {
            vm,
            i: 0,
            tych: TypChk::default(),
            arch,
        }
    }

    pub fn fun(&mut self, fun: Rc<Fun>) -> R<CmpFun> {
        /* new CmpFun */
        let mut f = CmpFun {
            args: fun.args.clone(),
            ret: fun.ret,
            asm: Asm::new(self.arch),
            tych: self.tych.clone(),
        };

        /* bind args */
        for Arg(n, t) in f.args.iter() {
            f.tych.bind(*n, *t);
        }

        Ok(f)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn new_CmpFun() {
    }
}
