use crate::{M::M, intern};

#[derive(Clone, Debug, PartialEq)]
pub struct Tup(pub &'static [M]);

impl Tup {
    pub fn new(ms: Vec<M>) -> Self {
        Self(intern::ms::add(ms))
    }
}
