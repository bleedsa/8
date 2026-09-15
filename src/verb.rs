use std::rc::Rc;
use crate::M::M;

pub mod pre {
    pub use super::{Dyds, Mons, Op, Dyd};
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Dyds {
    App,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Mons {
    Iota,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Op {
    Dyd(Dyds),
    Mon(Mons),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Dyd {
    op: Op,
    x: Rc<M>,
    y: Rc<M>,
}
