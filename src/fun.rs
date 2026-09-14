use std::fmt;
use crate::{intern, M::{MTy, M}};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Fun {
    pub names: &'static [&'static str],
    pub typs: &'static [MTy],
    pub body: &'static [M],
}

impl Fun {
    pub fn new<N>(args: Vec<(N, MTy)>, body: Vec<M>) -> Self
    where
        N: ToString,
    {
        let names = intern::strs::add(
            args.iter()
                .map(|(n, _)| intern::str::add(n.to_string()).as_str())
                .collect()
        );

        let typs = intern::tys::add(
            args.iter()
                .map(|(_, t)| *t)
                .collect()
        );

        let body = intern::bodies::add(body);
        Self {
            names,
            typs,
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


