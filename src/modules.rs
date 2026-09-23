/*!
 * compiled module structures and functions.
 */

use std::{rc::Rc, collections::HashMap};
use crate::{pre::*, cmp::CmpFun};

#[derive(Default, Clone)]
pub struct Mod<'m> {
    pub funs: HashMap<Name, Rc<CmpFun<'m>>>,
}
