# mapped-guard

[![Lib.rs](https://img.shields.io/badge/Lib.rs-*-84f)](https://lib.rs/crates/mapped-guard)
[![Crates.io](https://img.shields.io/crates/v/mapped-guard)](https://crates.io/crates/mapped-guard)
[![Docs.rs](https://docs.rs/mapped-guard/badge.svg)](https://docs.rs/crates/mapped-guard)

![Rust 1.40.0](https://img.shields.io/static/v1?logo=Rust&label=&message=1.40.0&color=grey)
[![CI](https://github.com/Tamschi/mapped-guard/workflows/CI/badge.svg?branch=develop)](https://github.com/Tamschi/mapped-guard/actions?query=workflow%3ACI+branch%3Adevelop)
![Crates.io - License](https://img.shields.io/crates/l/mapped-guard/0.0.1)

[![GitHub](https://img.shields.io/static/v1?logo=GitHub&label=&message=%20&color=grey)](https://github.com/Tamschi/mapped-guard)
[![open issues](https://img.shields.io/github/issues-raw/Tamschi/mapped-guard)](https://github.com/Tamschi/mapped-guard/issues)
[![open pull requests](https://img.shields.io/github/issues-pr-raw/Tamschi/mapped-guard)](https://github.com/Tamschi/mapped-guard/pulls)
[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/mapped-guard.svg)](https://web.crev.dev/rust-reviews/crate/mapped-guard/)

Returnable guards that represent for example a subset of the original borrow. Implemented for the standard guard types and easily extensible.

## Installation

Please use [cargo-edit](https://crates.io/crates/cargo-edit) to always add the latest version of this library:

```cmd
cargo add mapped-guard
```

## Example

```rust
use core::{fmt::Debug, ops::Deref};
use mapped_guard::MapGuard as _;
use std::sync::Mutex;

#[derive(Debug)]
struct Wrapper(usize);

let wrapper_mutex = Mutex::new(Wrapper(0));

/// # Panics
///
/// Iff `wrapper_mutex` is poisoned.
fn lock_increment(wrapper_mutex: &Mutex<Wrapper>) -> impl '_ + Deref<Target = usize> + Debug {
  let mut guard = wrapper_mutex.lock().unwrap();
  guard.0 += 1;
  guard.map_guard(|guard| &guard.0)
};

assert_eq!(
  *lock_increment(&wrapper_mutex),
  1
);
```

## A note on the implementation

This library is a workaround until mapped guards become available in the standard library and uses boxing internally.

While it's in practice likely possible to implement this more efficiently (without boxing) for many guards,
this isn't safe except for mutable borrows where the documentation explicitly states they are write-through.

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## [Code of Conduct](CODE_OF_CONDUCT.md)

## [Changelog](CHANGELOG.md)

## Versioning

`mapped-guard` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

* The minor version will not reset to 0 on major version changes (except for v1).  
Consider it the global feature level.
* The patch version will not reset to 0 on major or minor version changes (except for v0.1 and v1).  
Consider it the global patch level.

This includes the Rust version requirement specified above.  
Earlier Rust versions may be compatible, but this can change with minor or patch releases.

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).
