use std::{rc::Rc, fmt};
use crate::{intern, M::{MTy, M}};

#[derive(Copy, Clone, PartialEq)]
pub struct Arg<'a>(pub &'a str, pub MTy);

impl<'a> fmt::Debug for Arg<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fun {
    pub args: Rc<[Arg<'static>]>,
    pub body: Rc<[M]>,
}

impl Fun {
    pub fn new<N>(args: Vec<(N, MTy)>, body: Vec<M>) -> Self
    where
        N: ToString,
    {
        let args = args.iter()
            .map(|(n, t)| Arg(intern::str::add(n.to_string()), *t))
            .collect::<Vec<_>>()
            .into();
        Self {
            args,
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
