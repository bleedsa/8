use crate::pre::*;
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum MErr {
    InvalidVerb(Pos, String),
}

impl fmt::Display for MErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MErr::*;
        match self {
            InvalidVerb(p, s) => {
                write!(f, "{p}: invalid verb when constructing M: {s}")
            }
        }
    }
}

impl Error for MErr {}
