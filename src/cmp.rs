use std::{hint::likely, rc::Rc};
use crate::{M::M, vm::VM};

type Tape<'m> = &'m [Rc<M>];

pub struct Cmp<'m> {
    pub vm: &'m VM<'m>,
    pub tape: Tape<'m>,
    pub i: usize,
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
    pub fn new(vm: &'m VM<'m>, tape: Tape<'m>) -> Self {
        Self {
            vm,
            tape,
            i: 0,
        }
    }
}
