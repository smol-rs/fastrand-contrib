use core::ops::{Add, Mul, Neg};

use crate::BaseRng;

#[cfg(any(feature = "std", feature = "libm"))]
pub(super) fn f32(rng: &mut impl BaseRng, mu: f32, sigma: f32) -> f32 {
    float_normal_impl(rng, mu, sigma)
}

#[cfg(any(feature = "std", feature = "libm"))]
pub(super) fn f64(rng: &mut impl BaseRng, mu: f64, sigma: f64) -> f64 {
    float_normal_impl(rng, mu, sigma)
}

pub(super) fn f32_approx(rng: &mut impl BaseRng, mu: f32, sigma: f32) -> f32 {
    float_normal_approx_impl(rng, mu, sigma)
}

pub(super) fn f64_approx(rng: &mut impl BaseRng, mu: f64, sigma: f64) -> f64 {
    float_normal_approx_impl(rng, mu, sigma)
}

trait FloatExt:
    Add<Self, Output = Self> + Mul<Self, Output = Self> + Neg<Output = Self> + PartialOrd<Self> + Sized
{
    const EPSILON: Self;

    fn from_f64(x: f64) -> Self;
    #[cfg(any(feature = "std", feature = "libm"))]
    fn gen(rng: &mut impl BaseRng) -> Self;
}

#[cfg(any(feature = "std", feature = "libm"))]
trait FloatMathExt: FloatExt {
    const TAU: Self;

    fn ln(self) -> Self;
    fn sqrt(self) -> Self;
    fn cos(self) -> Self;
}

macro_rules! impl_float_ext {
    ($float:ident) => {
        impl FloatExt for $float {
            const EPSILON: Self = $float::EPSILON;

            #[inline]
            fn from_f64(x: f64) -> Self {
                x as $float
            }
            #[cfg(any(feature = "std", feature = "libm"))]
            #[inline]
            fn gen(rng: &mut impl BaseRng) -> Self {
                rng.$float()
            }
        }
    };
}

macro_rules! impl_float_math_ext {
    ($float:ident, $tau:ident) => {
        #[cfg(all(feature = "std", not(feature = "libm")))]
        impl FloatMathExt for $float {
            const TAU: Self = $tau;

            #[inline]
            fn ln(self) -> Self {
                $float::ln(self)
            }
            #[inline]
            fn sqrt(self) -> Self {
                $float::sqrt(self)
            }
            #[inline]
            fn cos(self) -> Self {
                $float::cos(self)
            }
        }

        #[cfg(feature = "libm")]
        impl FloatMathExt for $float {
            const TAU: Self = $tau;

            #[inline]
            fn ln(self) -> Self {
                libm_dep::Libm::<$float>::log(self)
            }
            #[inline]
            fn sqrt(self) -> Self {
                libm_dep::Libm::<$float>::sqrt(self)
            }
            #[inline]
            fn cos(self) -> Self {
                libm_dep::Libm::<$float>::cos(self)
            }
        }
    };
}

// TAU constant was stabilized in Rust 1.47. Our current MSRV is 1.43.
#[cfg(any(feature = "std", feature = "libm"))]
#[allow(clippy::excessive_precision)]
const F32_TAU: f32 = 6.28318530717958647692528676655900577_f32;
#[cfg(any(feature = "std", feature = "libm"))]
#[allow(clippy::excessive_precision)]
const F64_TAU: f64 = 6.28318530717958647692528676655900577_f64;

impl_float_ext!(f32);
impl_float_ext!(f64);
impl_float_math_ext!(f32, F32_TAU);
impl_float_math_ext!(f64, F64_TAU);

#[cfg(any(feature = "std", feature = "libm"))]
fn float_normal_impl<T: FloatMathExt>(rng: &mut impl BaseRng, mu: T, sigma: T) -> T {
    // https://en.wikipedia.org/wiki/Box%E2%80%93Muller_transform
    let u1 = loop {
        let u1 = T::gen(rng);

        if u1 > T::EPSILON {
            break u1;
        }
    };

    let u2 = T::gen(rng);
    let mag = sigma * (-T::from_f64(2.0) * u1.ln()).sqrt();
    mag * (T::TAU * u2).cos() + mu
}

fn float_normal_approx_impl<T: FloatExt>(rng: &mut impl BaseRng, mu: T, sigma: T) -> T {
    // http://marc-b-reynolds.github.io/distribution/2021/03/18/CheapGaussianApprox.html
    let u = rng.u128();

    // Counting ones in a u64 half of the generated number gives us binomial
    // distribution with p = 1/2 and n = 64. Subtracting 32 centers the
    // distribution on [-32, 32]. Shifting the lower u64 by 64 bits discard the
    // other half and `count_ones` can be used without any masking.
    let bd = (u << 64).count_ones() as i64 - 32;

    // Sample two u32 integers from uniform distribution.
    let a = ((u >> 64) & 0xffffffff) as i64;
    let b = (u >> 96) as i64;

    // First iteration of Central limit theorem (summing two uniform random
    // variables) _often_ gives triangular distribution. By using subtraction
    // instead of addition, the triangular distribution is centered around zero.
    let td = a - b;

    // Sum the binomial and triangular distributions.
    let r = ((bd << 32) + td) as f64;

    // Magic constant for scaling which is a result of minimizing the maximum
    // error with respect to the reference normal distribution.
    let k = 5.76917e-11;

    T::from_f64(k * r) * sigma + mu
}

#[cfg(test)]
mod tests {
    use fastrand::Rng;

    use super::*;

    fn normal_distribution_test<F>(sample: F)
    where
        F: Fn(&mut Rng, f32, f32) -> f32,
    {
        // The test based on the following picture from Wikipedia:
        // https://upload.wikimedia.org/wikipedia/commons/thumb/3/3a/Standard_deviation_diagram_micro.svg/1920px-Standard_deviation_diagram_micro.svg.png
        let mut rng = Rng::with_seed(42);

        let mu = 10.0;
        let sigma = 3.0;

        let total = 10000;
        let mut in_one_sigma_range = 0;
        let mut in_two_sigma_range = 0;
        let mut in_three_sigma_range = 0;
        for _ in 0..total {
            let value = sample(&mut rng, mu, sigma);

            if (mu - sigma..=mu + sigma).contains(&value) {
                in_one_sigma_range += 1;
            } else if (mu - sigma * 2.0..=mu + sigma * 2.0).contains(&value) {
                in_two_sigma_range += 1;
            } else if (mu - sigma * 3.0..=mu + sigma * 3.0).contains(&value) {
                in_three_sigma_range += 1;
            }
        }

        let in_one_sigma_range = in_one_sigma_range as f32 / total as f32 * 100.0;
        let in_two_sigma_range = in_two_sigma_range as f32 / total as f32 * 100.0;
        let in_three_sigma_range = in_three_sigma_range as f32 / total as f32 * 100.0;
        assert!(
            (64.0..=72.0).contains(&in_one_sigma_range),
            "value in \"one sigma range\" should be sampled ~68.2%, but is {}%",
            in_one_sigma_range
        );
        assert!(
            (23.0..=31.0).contains(&in_two_sigma_range),
            "value in \"two sigma range\" should be sampled ~27.2%, but is {}%",
            in_two_sigma_range
        );
        assert!(
            (1.0..=7.0).contains(&in_three_sigma_range),
            "value in \"three sigma range\" should be sampled ~4.2%, but is {}%",
            in_three_sigma_range
        );
    }

    #[test]
    #[cfg(any(feature = "std", feature = "libm"))]
    fn normal_is_actually_normal() {
        normal_distribution_test(float_normal_impl);
    }

    #[test]
    fn normal_approx_is_actually_normal() {
        normal_distribution_test(float_normal_approx_impl);
    }
}
