/*!
 * type class type checker.
 *
 * [typechecker zoo](
 *  https://sdiehl.github.io/typechecker-zoo/type-classes/implementation.html
 * )
 */

use crate::{
    A::A,
    M::MTy,
    typ::name::Name,
};
use std::collections::HashMap;

pub mod name;
pub mod err;

/** the repr for a type in the type system */
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Typ {
    /** a function arrow from x->y */
    Arrow(&'static Typ, &'static Typ),
    /** an atom type */
    Atom(MTy),
}
