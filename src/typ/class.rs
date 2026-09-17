/*!
 *  typeclasses.
 */

use std::collections::HashMap;
use crate::{typ::{Pred, Name, Typ}, A::A, fun::Fun, M::M};

/**
 * encoded info about a typeclass 
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: Name,
    pub ty: Name,
    pub supers: A<Name>,
    pub sigs: HashMap<Name, Typ>,
    pub defaults: HashMap<Name, M>,
}

/**
 * info about an instance of a typeclass
 */
#[derive(Clone, Debug, PartialEq)]
pub struct Instance {
    pub ctx: A<Pred>,
    pub head: Pred,
    pub methods: HashMap<Name, Fun>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Classes {
    pub classes: HashMap<Name, Class>,
    pub methods: HashMap<Name, Name>,
    pub instances: HashMap<Name, Instance>,
}
