use crate::pre::*;
use std::{fmt::Debug, ops::Index, rc::Rc};

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

impl<X> To<A<X>> for &[X]
where
    X: Clone + Debug + PartialEq,
{
    fn to(self) -> A<X> {
        A(self.into())
    }
}

impl<X> Index<usize> for A<X>
where
    X: Clone + Debug + PartialEq,
{
    type Output = X;

    #[inline(always)]
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}
