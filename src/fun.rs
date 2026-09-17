use crate::{
    M::{M, MTy},
    intern,
};
use std::{fmt, rc::Rc};

#[derive(Copy, Clone, PartialEq)]
pub struct Arg<'a>(pub &'a str, pub MTy);

impl<'a> fmt::Debug for Arg<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fun {
    pub arrows: Rc<[Arg<'static>]>,
    pub body: Rc<[M]>,
}

impl Fun {
    pub fn new<N>(arrows: Vec<(N, MTy)>, body: Vec<M>) -> Self
    where
        N: ToString,
    {
        let arrows = arrows
            .iter()
            .map(|(n, t)| Arg(intern::str::add(n.to_string()), *t))
            .collect::<Vec<_>>()
            .into();
        Self {
            arrows,
            body: body.into(),
        }
    }
}

impl fmt::Display for Fun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.body
                .iter()
                .map(|x| format!("{x:?}"))
                .intersperse(";".to_string())
                .collect::<String>()
        )
    }
}
