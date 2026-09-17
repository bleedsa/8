/*!
 * type class type checker.
 *
 * [typechecker zoo](
 *  https://sdiehl.github.io/typechecker-zoo/type-classes/implementation.html
 * )
 */

use crate::{M::MTy, A::A, typ::{name::Name, class::Classes}};
use std::collections::HashMap;

pub mod class;
pub mod name; 

/** the repr for a type in the type system */
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Typ {
    /** wraps a type variable name */
    Var(Name),
    /** a type constructor name paired with a vec of argument types */
    Con(Name, A<Typ>),
    /** a function arrow from x->y */
    Arrow(&'static Typ, &'static Typ),
    /** an atom type */
    Atom(MTy),
}

/**
 * a type predicate.
 *
 * `name`: name of the class being asserted by the predicate.
 * `ty`: the type the assertion is about.
 */
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pred {
    pub class: Name,
    pub ty: Typ,
}

/** 
 * a qualified type contains a type and a list of predicates to check against
 *
 * the preds field is the context of predicates to check against.
 */ 
#[derive(Clone, Debug, PartialEq)]
pub struct Qual {
    pub ty: Typ,
    pub preds: A<Pred>,
}

/**
 * a scheme quantifies over type variables with a qualified body.
 *
 * the scheme `forall a. Eq a => a -> a -> bool` is
 * ```
 * Scheme {
 *      vars: ["a"],
 *      qual: Qual {
 *          preds: [Eq a],
 *          ty: a -> a -> bool,
 *      }
 * }
 * ```
 */
#[derive(Clone, Debug, PartialEq)]
pub struct Scheme {
    pub qual: Qual,
    pub vars: A<Name>,
}

pub struct Env(pub HashMap<Name, Scheme>);

pub struct Checker {
    pub env: Env,
    pub classes: Classes,
}

impl Checker {
    /**
     * return type is a quad of substitution, type, pending Preds, and an elabo-
     * rate core term.
     */
    pub fn infer_expr(&mut self) {
        
    }
}
