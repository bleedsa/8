/*!
 * jit compilation
 */

use crate::{M::{M, MTy}, pre::*, vm::VM, typ::{Typ, TypChk}, asm::Asm};
use std::{hint::likely, rc::Rc};

pub mod err;

type Tape<'m> = &'m [Rc<M>];

#[derive(PartialEq)]
pub struct CmpFun<'m> {
    pub args: &'m [MTy],
    pub ret: MTy,
    pub asm: Asm,
}

pub struct Cmp<'m> {
    pub vm: &'m mut VM<'m>,
    pub tape: Tape<'m>,
    pub i: usize,
    pub tych: TypChk,
}

impl<'m> Iterator for Cmp<'m> {
    type Item = Rc<M>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        let t = self.tape;
        if likely(i < t.len()) {
            self.i += 1;
            Some(t[i].clone())
        } else {
            None
        }
    }
}

impl<'m> Cmp<'m> {
    #[inline(always)]
    pub fn new(vm: &'m mut VM<'m>, tape: Tape<'m>) -> Self {
        Self { vm, tape, i: 0, tych: TypChk::default() }
    }

    pub fn cmp(&mut self) -> R<()> {
        for m in &mut *self {
            match self.tych.typ_of(m.clone())? {
                Typ::Atom(MTy::Int) => todo!(),
                _ => return err_cmp!(CantCmpTy((*m).clone())),
            }
        }

        Ok(())
    }
}
