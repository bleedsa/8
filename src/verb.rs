use std::rc::Rc;
use crate::M::M;

pub mod pre {
    pub use super::{Dyds, Mons, Mon, Dyd};
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Dyds {
    App,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Dyd {
    v: Dyds,
    x: Rc<M>,
    y: Rc<M>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Mons {
    Iota,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mon {
    v: Mons,
    x: Rc<M>,
    y: Rc<M>,
}
