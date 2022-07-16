# Fornjot Plugins

[![Continuous integration](https://github.com/Michael-F-Bryan/fornjot-plugins/workflows/Continuous%20integration/badge.svg?branch=master)](https://github.com/Michael-F-Bryan/fornjot-plugins/actions)

([API Docs])

A proof-of-concept plugin system for [Fornjot][fornjot].

## Getting Started

After you've installed [the Rust compiler][rustup] and cloned this repo, you
will want to compile a model.

```console
$ cargo build --release --package cuboid
$ ls target/release/*.so
target/release/libcuboid.so
```

Once you've done that, Fornjot should be able to load it.

```console
$ fj-app --model ./models/cuboid
```

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE.md) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT.md) or
   http://opensource.org/licenses/MIT)

at your option.

It is recommended to always use [cargo-crev][crev] to verify the
trustworthiness of each of your dependencies, including this one.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

The intent of this crate is to be free of soundness bugs. The developers will
do their best to avoid them, and welcome help in analysing and fixing them.

[API Docs]: https://michael-f-bryan.github.io/fornjot-plugins
[crev]: https://github.com/crev-dev/cargo-crev
[fornjot]: https://www.fornjot.app/
[rustup]: https://rustup.rs/
