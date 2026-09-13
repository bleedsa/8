use crate::pre::*;
use std::{mem::MaybeUninit, sync::Mutex};

pub struct InternEntry<X>(pub X);

impl<X> InternEntry<X> {
    #[inline(always)]
    pub fn new(x: X) -> Self {
        Self(x)
    }
}

impl<X> AsRef<X> for InternEntry<X> {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a X {
        &self.0
    }
}

pub struct Intern<X> {
    lock: Mutex<Vec<InternEntry<X>>>,
}

impl<X> Intern<X> {
    pub fn new() -> Self {
        Self {
            lock: Mutex::new(Vec::new()),
        }
    }

    pub fn add<'a>(&'a mut self, x: X) -> &'a X {
        /* lock w */
        let k = match self.lock.get_mut() {
            Ok(x) => x,
            Err(e) => fatal!("POISONED MUTEX: {e}"),
        };

        /* push */
        let v = &mut *k;
        let L = v.len();
        v.push(InternEntry::new(x));

        /* ref */
        (&(*k)[L]).as_ref()
    }
}

pub static mut POS: MaybeUninit<Intern<Pos>> = MaybeUninit::uninit();

pub fn init() {
    unsafe {
        POS = MaybeUninit::new(Intern::new());
    }
}

pub fn pos() -> &'static mut Intern<Pos> {
    unsafe { &mut *((&raw mut POS) as *mut Intern<Pos>) }
}
