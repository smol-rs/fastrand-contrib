//! Extension functionality for the [`fastrand`] crate.
//!
//! [`fastrand`]: https://crates.io/crates/fastrand

use core::ops::RangeBounds;

pub use fastrand::{self, Rng};

trait BaseRng {
    fn f32(&mut self) -> f32;
    fn f64(&mut self) -> f64;
}

impl BaseRng for Rng {
    #[inline]
    fn f32(&mut self) -> f32 {
        Rng::f32(self)
    }
    #[inline]
    fn f64(&mut self) -> f64 {
        Rng::f64(self)
    }
}

#[cfg(feature = "std")]
struct GlobalRng;

#[cfg(feature = "std")]
impl BaseRng for GlobalRng {
    #[inline]
    fn f32(&mut self) -> f32 {
        fastrand::f32()
    }
    #[inline]
    fn f64(&mut self) -> f64 {
        fastrand::f64()
    }
}

macro_rules! define_ext {
    ($(
        fn $name:ident(&mut self, $($argname:ident:$argty:ty),*) -> $ret:ty $bl:block
    )*) => {
        pub trait RngExt {
            $(
            fn $name(&mut self, $($argname: $argty),*) -> $ret;
            )*
        }

        impl RngExt for Rng {
            $(
            fn $name(&mut self, $($argname: $argty),*) -> $ret $bl
            )*
        }

        $(
        #[cfg(feature = "std")]
        pub fn $name($($argname:$argty),*) -> $ret {
            impl GlobalRng {
                fn $name(&mut self, $($argname:$argty),*) -> $ret $bl
            }

            GlobalRng::$name(&mut GlobalRng, $($argname),*)
        }
        )*
    }
}

define_ext! {
    fn f32_range(&mut self, range: impl RangeBounds<f32>) -> f32 {
        let _ = range;
        todo!()
    }

    fn f64_range(&mut self, range: impl RangeBounds<f64>) -> f64 {
        let _ = range;
        todo!()
    }
}

mod __private {
    #[doc(hidden)]
    pub trait Sealed {}
    impl Sealed for fastrand::Rng {}
}
