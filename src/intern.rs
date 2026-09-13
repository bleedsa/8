use dtor::dtor;
use crate::pre::*;
use std::{sync::Mutex, ptr, hint::unlikely, cell::UnsafeCell, mem::ManuallyDrop, marker::PhantomData};

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

pub struct Intern<'a, X> {
    pub ptr: *mut InternEntry<X>,
    pub cap: usize,
    pub len: usize,
    pub _mark: PhantomData<&'a ()>,
}

impl<'a, X> Intern<'a, X> {
    pub const fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
            cap: 0,
            len: 0,
            _mark: PhantomData,
        }
    }

    fn alloc(&mut self) {
        println!("alloc()");
        self.cap = 8;
        unsafe {
            self.ptr = xxx::new(self.cap).expect("Intern::alloc(): failed to alloc");
        }
    }

    pub fn grow(&mut self) {
        unsafe {
            if self.cap == 0 {
                self.alloc();
            }

            let cap = self.cap * 2;
            let ptr: *mut InternEntry<X> = xxx::new(cap).expect("Intern::grow(): failed to realloc");

            /* copy */
            for i in 0..self.cap {
                ptr::write(ptr.add(i), ptr::read(self.ptr.add(i)));
            }
            
            self.cap = cap;
            self.ptr = ptr;
        }
    }

    pub fn add(&'a mut self, x: X) -> &'a X
    where
        X: PartialEq
    {
         if unlikely(self.cap == 0) {
            self.alloc();
        }

        if unlikely(self.len >= self.cap) {
            self.grow();
        }

        for i in 0..self.len {
            let p = unsafe { (*self.ptr.add(i)).as_ref() };
            if p == &x {
                return p;
            }
        }

        unsafe {
            let p = self.ptr.add(self.len);
            ptr::write(p, InternEntry::new(x));
            self.len += 1;
            (&*p).as_ref()
        }
    }

}

impl<'a, X> Drop for Intern<'a, X> {
    fn drop(&mut self) {
        unsafe {
            xxx::free(self.ptr, self.cap);
            self.ptr = ptr::null_mut();
        }
    }
}

pub struct SIntern<X> {
    pub tab: ManuallyDrop<UnsafeCell<Intern<'static, X>>>,
    pub lock: ManuallyDrop<Mutex<()>>,
}

impl<X> SIntern<X> {
    pub const fn new() -> Self {
        Self {
            tab: ManuallyDrop::new(Intern::new().into()),
            lock: ManuallyDrop::new(Mutex::new(())),
        }
    }

    pub fn add<'a>(&'a mut self, x: X) -> &'a X
    where
        'a: 'static,
        X: PartialEq,
    {
        let d = self.lock.lock();
        let r = (*self.tab.get_mut()).add(x);
        drop(d);
        r
    }
}

impl<X> Drop for SIntern<X> {
    fn drop(&mut self) {
        ()
    }
}

unsafe impl<X> Send for SIntern<X> {}
unsafe impl<X> Sync for SIntern<X> {}


pub mod pos {
    use super::*;

    pub static mut POS: ManuallyDrop<SIntern<Pos>> = ManuallyDrop::new(SIntern::new());

    pub fn add(x: Pos) -> &'static Pos {
        unsafe {
            (&mut *&raw mut POS).add(x)
        }
    }
}

#[dtor(unsafe)]
pub fn deinit() {
    unsafe {
        ManuallyDrop::drop(&mut pos::POS);
    }
}
