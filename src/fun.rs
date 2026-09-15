use std::fmt;
use crate::{intern, M::{MTy, M}};

#[derive(Copy, Clone, PartialEq)]
pub struct Arg<'a>(pub &'a str, pub MTy);

impl<'a> fmt::Debug for Arg<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Fun {
    pub args: &'static [Arg<'static>],
    pub body: &'static [M],
}

impl Fun {
    pub fn new<N>(args: Vec<(N, MTy)>, body: Vec<M>) -> Self
    where
        N: ToString,
    {
        let args = args.iter()
            .map(|(n, t)| Arg(intern::str::add(n.to_string()), *t))
            .collect();
        let args = intern::args::add(args);
        let body = intern::ms::add(body);
        Self {
            args,
            body,
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
