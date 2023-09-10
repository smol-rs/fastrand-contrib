# fastrand

[![Build](https://github.com/smol-rs/fastrand/workflows/Build%20and%20test/badge.svg)](
https://github.com/smol-rs/fastrand/actions)
[![License](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](
https://github.com/smol-rs/fastrand)
[![Cargo](https://img.shields.io/crates/v/fastrand.svg)](
https://crates.io/crates/fastrand)
[![Documentation](https://docs.rs/fastrand/badge.svg)](
https://docs.rs/fastrand)

Extension functionality for the [`fastrand`] crate.

This crate contains code that may be of some use to users of [`fastrand`]. Code contained in
this crate is not included in [`fastrand`] due to either the niche not being large enough to
justify the new functionality or for semver concerns.

## Usage

Various functions are exposed in this crate as top-level functions. These manipulate the global
thread-local RNG and can be used without any local state. Note that these require the `"std"`
default feature to be enabled.

```
use fastrand_contrib::f32_range;

let x = f32_range(1.5..3.0);
assert!(x >= 1.5 && x < 3.0);
```

To extend [`fastrand::Rng`], import the [`RngExt`] trait.

```
use fastrand_contrib::RngExt;
```

Now, all new methods are available on [`fastrand::Rng`].

```
use fastrand::Rng;
use fastrand_contrib::RngExt;

let mut rng = Rng::with_seed(0x1234);
let x = rng.f32_range(1.5..3.0);
assert!(x >= 1.5 && x < 3.0);
```

[`fastrand`]: https://crates.io/crates/fastrand
[`fastrand::Rng`]: https://docs.rs/fastrand/latest/fastrand/struct.Rng.html

# Features

- `std` (enabled by default): Enables the `std` library. Freestanding functions only work with this feature enabled. Also enables the `fastrand/std` feature.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
