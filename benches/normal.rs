#![feature(test)]

extern crate test;

use fastrand::Rng;
use fastrand_contrib::RngExt;
use test::Bencher;

const SEED: u64 = 42;
const MU: f64 = 10.0;
const SIGMA: f64 = 3.0;

#[bench]
fn box_muller(b: &mut Bencher) {
    let mut rng = Rng::with_seed(SEED);

    b.iter(|| {
        let mu = core::hint::black_box(MU);
        let sigma = core::hint::black_box(SIGMA);

        // https://en.wikipedia.org/wiki/Box%E2%80%93Muller_transform
        let u1 = loop {
            let u1 = rng.f64();

            if u1 > f64::EPSILON {
                break u1;
            }
        };

        let u2 = rng.f64();
        let mag = sigma * (-2.0 * u1.ln()).sqrt();
        let output = mag * (core::f64::consts::TAU * u2).cos() + mu;

        core::hint::black_box(output);
    });
}

#[bench]
fn standard_approximation(b: &mut Bencher) {
    let mut rng = Rng::with_seed(SEED);

    b.iter(|| {
        let mu = core::hint::black_box(MU);
        let sigma = core::hint::black_box(SIGMA);

        // http://marc-b-reynolds.github.io/distribution/2021/03/18/CheapGaussianApprox.html
        let u = rng.u128(..);

        let mask = 0xffffffff;
        let a = (u & mask) as i64;
        let b = ((u >> 32) & mask) as i64;
        let c = ((u >> 64) & mask) as i64;
        let d = (u >> 96) as i64;

        // Magic constant.
        let k = 3.97815e-10;

        let output = k * ((a + b) - (c + d)) as f64 * sigma + mu;

        core::hint::black_box(output);
    });
}

#[bench]
fn popcount_approximation(b: &mut Bencher) {
    let mut rng = Rng::with_seed(SEED);

    b.iter(|| {
        let mu = core::hint::black_box(MU);
        let sigma = core::hint::black_box(SIGMA);

        // http://marc-b-reynolds.github.io/distribution/2021/03/18/CheapGaussianApprox.html
        let u = rng.u128(..);

        let bd = (u << 64).count_ones() as i64 - 32;

        let a = ((u >> 64) & 0xffffffff) as i64;
        let b = (u >> 96) as i64;

        let td = a - b;

        let r = ((bd << 32) + td) as f64;

        // Magic constant.
        let k = 5.76917e-11;

        let output = k * r * sigma + mu;

        core::hint::black_box(output);
    })
}

#[bench]
fn f64_normal(b: &mut Bencher) {
    let mut rng = Rng::with_seed(SEED);

    b.iter(|| {
        let mu = core::hint::black_box(MU);
        let sigma = core::hint::black_box(SIGMA);

        let output = rng.f64_normal(mu, sigma);

        core::hint::black_box(output);
    });
}

#[bench]
fn f64_normal_approx(b: &mut Bencher) {
    let mut rng = Rng::with_seed(SEED);

    b.iter(|| {
        let mu = core::hint::black_box(MU);
        let sigma = core::hint::black_box(SIGMA);

        let output = rng.f64_normal_approx(mu, sigma);

        core::hint::black_box(output);
    });
}
