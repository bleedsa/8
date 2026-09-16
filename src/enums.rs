pub mod pre {
    pub use crate::{enum_jmps, enums::EnumJmpTab};
    pub use std::ops::Index;
}

pub struct EnumJmpTab<T, const Z: usize>(pub [T; Z]);

#[macro_export]
macro_rules! enum_jmps {
    {
        enum $e:ident
        derives ($($D:ident),*)
        {
            $($n:ident),*$(,)*
        }
        $(
            static $s:ident: fn($($a:ty),*) -> $r:ty
                = [$([$i:ident]=$x:expr),*$(,)*];
        )*
    } => {
        #[derive($($D),*)]
        pub enum $e {
            $($n = ${index()}),*
        }

        impl $e {
            pub const fn num() -> usize {
                ${count($n)}
            }
        }

        impl Into<usize> for $e {
            fn into(self) -> usize {
                self as usize
            }
        }

        impl<T, const Z: usize> Index<$e> for EnumJmpTab<T, Z> {
            type Output = T;

            fn index(&self, index: $e) -> &T {
                &self.0[Into::<usize>::into(index)]
            }
        }

        $(
            pub static $s: EnumJmpTab<fn($($a),*) -> $r, {$e::num()}> = EnumJmpTab({
                type FN = fn($($a),*) -> $r;
                let mut r = [MaybeUninit::uninit(); $e::num()];
                unsafe {
                    $(
                        r[$e::$i as usize] = MaybeUninit::new($x as FN);
                    )*
                    *(r.as_ptr() as *const [FN; $e::num()])
                }
            });
        )*
    };
}
