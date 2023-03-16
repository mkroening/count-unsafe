# count-unsafe

[![Crates.io](https://img.shields.io/crates/v/count-unsafe)](https://crates.io/crates/count-unsafe)
[![CI](https://github.com/mkroening/count-unsafe/actions/workflows/ci.yml/badge.svg)](https://github.com/mkroening/count-unsafe/actions/workflows/ci.yml)

Count-unsafe counts the amount of unsafe Rust code in a given path.

This project is built on the [geiger] library.
In contrast to [cargo-geiger] though, this application does not integrate with cargo and simply counts unsafe code in all Rust source files in a given path.

[geiger]: https://crates.io/crates/geiger
[cargo-geiger]: https://crates.io/crates/cargo-geiger

## Installation

This project is available on [crates.io]:

[crates.io]: https://crates.io/crates/count-unsafe

```console
cargo install count-unsafe
```

## Example

Running count-unsafe on Cargo's source ([0.69.0]):

[0.69.0]: https://github.com/rust-lang/cargo/tree/0.69.0

```console
$ count-unsafe cargo/src
{
  "functions": {
    "safe": 759,
    "unsafe_": 2
  },
  "exprs": {
    "safe": 50434,
    "unsafe_": 238
  },
  "item_impls": {
    "safe": 549,
    "unsafe_": 0
  },
  "item_traits": {
    "safe": 16,
    "unsafe_": 0
  },
  "methods": {
    "safe": 1804,
    "unsafe_": 0
  }
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
