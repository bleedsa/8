use assembler::ExecutableAnonymousMemoryMap;

use crate::pre::*;

pub struct VM {
    pub map: ExecutableAnonymousMemoryMap,
}

impl VM {
    pub fn new() -> R<Self> {
        Ok(Self {
            map: ExecutableAnonymousMemoryMap::new(4096, true, false)?,
        })
    }
}
