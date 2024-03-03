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
//! - `libm`: Uses [`libm`] dependency for math functions in `no_std` environment.
//!
//! Note that some functions are not available in `no_std` context if `libm` feature is not enabled.
//!
//! [`fastrand`]: https://crates.io/crates/fastrand
//! [`fastrand::Rng`]: https://docs.rs/fastrand/latest/fastrand/struct.Rng.html
//! [`libm`]: https://crates.io/crates/libm

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code, future_incompatible, missing_docs)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/smol-rs/smol/master/assets/images/logo_fullsize_transparent.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/smol-rs/smol/master/assets/images/logo_fullsize_transparent.png"
)]

mod float_normal;
mod float_range;

use core::ops::RangeBounds;

pub use fastrand::{self, Rng};

trait BaseRng {
    fn f32(&mut self) -> f32;
    fn f64(&mut self) -> f64;
    fn bool(&mut self) -> bool;
    fn u128(&mut self) -> u128;
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
    #[inline]
    fn u128(&mut self) -> u128 {
        Rng::u128(self, ..)
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
    #[inline]
    fn u128(&mut self) -> u128 {
        fastrand::u128(..)
    }
}

macro_rules! define_ext {
    ($(
        $(#[$meta:meta])*
        fn $name:ident(&mut self, $($argname:ident:$argty:ty),*) -> $ret:ty => $imp:path;
    )*) => {
        /// Extra methods for [`fastrand::Rng`].
        pub trait RngExt: __private::Sealed {
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
        impl GlobalRng {
            $(#[$meta])*
            fn $name(&mut self, $($argname:$argty),*) -> $ret {
                $imp(self, $($argname),*)
            }
        }
        #[cfg(feature = "std")]
        $(#[$meta])*
        pub fn $name($($argname:$argty),*) -> $ret {
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

    /// Generate a 32-bit floating point number in the normal distribution with
    /// mean mu and standard deviation sigma.
    #[cfg(any(feature = "std", feature = "libm"))]
    fn f32_normal(&mut self, mu: f32, sigma: f32) -> f32 => float_normal::f32;

    /// Generate a 64-bit floating point number in the normal distribution with
    /// mean mu and standard deviation sigma.
    #[cfg(any(feature = "std", feature = "libm"))]
    fn f64_normal(&mut self, mu: f64, sigma: f64) -> f64 => float_normal::f64;

    /// Generate a 32-bit floating point number in the normal distribution with
    /// mean mu and standard deviation sigma using an approximation algorithm.
    fn f32_normal_approx(&mut self, mu: f32, sigma: f32) -> f32 => float_normal::f32_approx;

    /// Generate a 64-bit floating point number in the normal distribution with
    /// mean mu and standard deviation sigma using an approximation algorithm.
    fn f64_normal_approx(&mut self, mu: f64, sigma: f64) -> f64 => float_normal::f64_approx;
}

mod __private {
    #[doc(hidden)]
    pub trait Sealed {}
    impl Sealed for fastrand::Rng {}
}
