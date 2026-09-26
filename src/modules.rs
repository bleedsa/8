/*!
 * compiled module structures and functions.
 */

use crate::{M::M, cmp::CmpFun, pre::*};
use std::{collections::HashMap, rc::Rc};

#[derive(Default, Clone)]
pub struct Mod {
    pub funs: HashMap<Name, Rc<CmpFun>>,
    pub binds: HashMap<Name, Rc<M>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_module() -> R<()> {
        Ok(())
    }
}
