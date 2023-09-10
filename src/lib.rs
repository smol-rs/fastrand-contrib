//! Extension functionality for the [`fastrand`] crate.
//!
//! This crate contains code that may be of some use to users of [`fastrand`]. Code contained in
//! this crate is not included in [`fastrand`] due to either the niche not being large enough to
//! justify the new functionality or for semver concerns.
//!
//! ## Usage
//!
//! Various functions are exposed in this crate as top-level functions. These manipulate the global
//! thread-local RNG and can be used without any local state. Note that these require the `"std"`
//! default feature to be enabled.
//!
//! ```
//! # #[cfg(feature = "std")] {
//! use fastrand_contrib::f32_range;
//!
//! let x = f32_range(1.5..3.0);
//! assert!(x >= 1.5 && x < 3.0);
//! # }
//! ```
//!
//! To extend [`fastrand::Rng`], import the [`RngExt`] trait.
//!
//! ```
//! use fastrand_contrib::RngExt;
//! ```
//!
//! Now, all new methods are available on [`fastrand::Rng`].
//!
//! ```
//! use fastrand::Rng;
//! use fastrand_contrib::RngExt;
//!
//! let mut rng = Rng::with_seed(0x1234);
//! let x = rng.f32_range(1.5..3.0);
//! assert!(x >= 1.5 && x < 3.0);
//! ```
//! # Features
//!
//! - `std` (enabled by default): Enables the `std` library. Freestanding functions only work with this
//!   feature enabled. Also enables the `fastrand/std` feature.
//!
//! [`fastrand`]: https://crates.io/crates/fastrand
//! [`fastrand::Rng`]: https://docs.rs/fastrand/latest/fastrand/struct.Rng.html

#![cfg_attr(not(feature = "std"), no_std)]

mod float_range;

use core::ops::RangeBounds;

pub use fastrand::{self, Rng};

trait BaseRng {
    fn f32(&mut self) -> f32;
    fn f64(&mut self) -> f64;
    fn bool(&mut self) -> bool;
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
    #[inline]
    fn bool(&mut self) -> bool {
        Rng::bool(self)
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
    #[inline]
    fn bool(&mut self) -> bool {
        fastrand::bool()
    }
}

macro_rules! define_ext {
    ($(
        $(#[$meta:meta])*
        fn $name:ident(&mut self, $($argname:ident:$argty:ty),*) -> $ret:ty => $imp:path;
    )*) => {
        /// Extra methods for [`fastrand::Rng`].
        pub trait RngExt {
            $(
            $(#[$meta])*
            fn $name(&mut self, $($argname: $argty),*) -> $ret;
            )*
        }

        impl RngExt for Rng {
            $(
            $(#[$meta])*
            fn $name(&mut self, $($argname: $argty),*) -> $ret {
                $imp(self, $($argname),*)
            }
            )*
        }

        $(
        #[cfg(feature = "std")]
        $(#[$meta])*
        pub fn $name($($argname:$argty),*) -> $ret {
            impl GlobalRng {
                $(#[$meta])*
                fn $name(&mut self, $($argname:$argty),*) -> $ret {
                    $imp(self, $($argname),*)
                }
            }

            GlobalRng::$name(&mut GlobalRng, $($argname),*)
        }
        )*
    }
}

define_ext! {
    /// Generate a 32-bit floating point number in the specified range.
    fn f32_range(&mut self, range: impl RangeBounds<f32>) -> f32 => float_range::f32;

    /// Generate a 64-bit floating point number in the specified range.
    fn f64_range(&mut self, range: impl RangeBounds<f64>) -> f64 => float_range::f64;
}

mod __private {
    #[doc(hidden)]
    pub trait Sealed {}
    impl Sealed for fastrand::Rng {}
}
