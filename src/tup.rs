use std::rc::Rc;
use crate::M::M;

#[derive(Clone, Debug, PartialEq)]
pub struct Tup(pub Rc<[M]>);

impl Tup {
    pub fn new(ms: Vec<M>) -> Self {
        Self(ms.into())
    }
}
