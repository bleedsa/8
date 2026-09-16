use std::{fmt::Debug, rc::Rc};
use crate::pre::*;

#[derive(Clone, Debug, PartialEq)]
pub struct A<X>(pub Rc<[X]>)
where
    X: Clone + Debug + PartialEq;

impl<X> To<A<X>> for Vec<X>
where
    X: Clone + Debug + PartialEq,
{
    fn to(self) -> A<X> {
        A(self.into())
    }
}

impl<X> To<Vec<X>> for A<X>
where
    X: Clone + Debug + PartialEq,
{
    fn to(self) -> Vec<X> {
        (&*self.0).into()
    }
}
