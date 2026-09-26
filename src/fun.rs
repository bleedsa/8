/*!
 *  M::M function repr.
 */

use crate::{
    A::A,
    M::{M, MTy},
    pre::*,
    typ::Typ,
};
use std::{fmt, rc::Rc};

#[derive(Copy, Clone, PartialEq)]
pub struct Arg(pub Name, pub Typ);

impl fmt::Debug for Arg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.0, self.1)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fun {
    pub args: A<Arg>,
    pub body: A<Rc<M>>,
    pub ret: Typ,
}

#[test]
fn construct_add_fun() {
    use crate::V;
    let args = vec![("x", Typ::Atom(MTy::Int)), ("y", Typ::Atom(MTy::Int))];
    let body = vec![V!(Add, Some("x"), Some("y"))];
    let _ = Fun::new(args, body, Typ::Atom(MTy::Int));
}

impl Fun {
    pub fn new<N>(args: Vec<(N, Typ)>, body: Vec<Rc<M>>, ret: Typ) -> Self
    where
        N: AsRef<str>,
    {
        Self {
            args: args
                .iter()
                .map(|(n, t)| Arg(Name::named(n.as_ref()), *t))
                .collect::<Vec<_>>()
                .to(),
            body: body.to(),
            ret,
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
