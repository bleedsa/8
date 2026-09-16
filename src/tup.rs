use crate::M::M;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Tup(pub Rc<[M]>);

impl Tup {
    pub fn new(ms: Vec<M>) -> Self {
        Self(ms.into())
    }
}
