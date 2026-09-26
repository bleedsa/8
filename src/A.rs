/*!
 * static, refcounted vectors
 */

use crate::pre::*;
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::Index,
    rc::Rc,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct A<X>(pub Rc<[X]>)
where
    X: Clone + PartialEq;

impl<X> A<X>
where
    X: Clone + Debug + PartialEq,
{
    pub fn iter(&self) -> impl Iterator<Item = &X> {
        self.0.iter()
    }
}

impl<X> To<A<X>> for Vec<X>
where
    X: Clone + PartialEq,
{
    fn to(self) -> A<X> {
        A(self.into())
    }
}

impl<X> To<Vec<X>> for A<X>
where
    X: Clone + PartialEq,
{
    fn to(self) -> Vec<X> {
        (&*self.0).into()
    }
}

impl<X> To<A<X>> for &[X]
where
    X: Clone + PartialEq,
{
    fn to(self) -> A<X> {
        A(self.into())
    }
}

impl<X> Index<usize> for A<X>
where
    X: Clone + PartialEq,
{
    type Output = X;

    #[inline(always)]
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl<X> Hash for A<X>
where
    X: Clone + PartialEq + Hash,
{
    fn hash<H>(&self, s: &mut H)
    where
        H: Hasher,
    {
        (&*self.0).hash(s)
    }
}
