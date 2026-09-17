/*!
 * type class type checker.
 *
 * [typechecker zoo](
 *  https://sdiehl.github.io/typechecker-zoo/type-classes/implementation.html
 * )
 */

use crate::{M::MTy, intern, pre::*, A::A, typ::class::Classes};
use std::{collections::HashMap, sync::Mutex};

pub mod class;

type TmpVal = u16;
static TMP_COUNT: Mutex<TmpVal> = Mutex::new(0);

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
#[repr(u8)]
pub enum Name {
    Tmp(TmpVal),
    Named(&'static str),
}

impl Name {
    /** make a named variable */
    #[inline(always)]
    pub fn named<S>(s: S) -> Self
    where
        S: ToString,
    {
        Self::Named(intern::str::add(s.to_string()))
    }

    /** make a temporary variable */
    #[inline(always)]
    pub fn tmp() -> Self {
        let mut G = match TMP_COUNT.lock() {
            Ok(G) => G,
            Err(e) => fatal!("Name::tmp(): poisoned mutex: {e}"),
        };
        let v = *G;

        *G += 1;
        Self::Tmp(v)
    }
}

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
