# Fornjot Plugins

[![Continuous integration](https://github.com/Michael-F-Bryan/fornjot-plugins/workflows/Continuous%20integration/badge.svg?branch=master)](https://github.com/Michael-F-Bryan/fornjot-plugins/actions)

([API Docs][docs])

A proof-of-concept plugin system for [Fornjot][fornjot].

## Getting Started

After you've installed [the Rust compiler][rustup] and cloned this repo, you
can point the Fornjot app at the *Cuboid* model and use it directly.

```console
$ fj-app --model ./models/cuboid
```

To load a model that was compiled to WebAssembly, first cross-compile your
model to `wasm32-unknown-unknown`.

```console
$ cargo build --package cuboid --target wasm32-unknown-unknown
$ ls target/wasm32-unknown-unknown/debug/*.wasm
target/wasm32-unknown-unknown/debug/cuboid.wasm
```

Next, point the `$FJ_WASM_PLUGIN` environment variable at our compiled
WebAssembly module.

```console
$ export FJ_WASM_PLUGIN=$PWD/target/wasm32-unknown-unknown/debug/cuboid.wasm
```

The plugin we compiled to WebAssembly can now be used via the `wasm-shim` model.
This is a special crate that compiles to a normal `*.so` plugin, but whenever
Fornjot asks it to generate geometry, the `wasm-shim` will delegate its
implementation to the WebAssembly plugin it loaded.

```console
$ fj-app --model crates/wasm-shim
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
2022-07-17T11:51:56.772937Z DEBUG fj_wasm_shim: Reading WebAssembly from disk path=/home/michael/Documents/fornjot-plugins/target/wasm32-unknown-unknown/debug/cuboid.wasm
2022-07-17T11:51:56.776027Z DEBUG fj_wasm_shim: Parsing the WebAssembly module
2022-07-17T11:51:58.246512Z DEBUG fj_wasm_shim: Instantiated the WebAssembly module
2022-07-17T11:51:58.246871Z DEBUG fj_wasm_shim: Initializing the plugin
2022-07-17T11:51:58.247076Z  INFO init: cuboid: Registered cuboid
2022-07-17T11:51:58.261879Z DEBUG shape: cuboid: Creating a cuboid model x=3.0 y=2.0 z=1.0
```

Consult the [API docs][docs] for more detail on how the plugin system works.

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

[docs]: https://michael-f-bryan.github.io/fornjot-plugins
[crev]: https://github.com/crev-dev/cargo-crev
[fornjot]: https://www.fornjot.app/
[rustup]: https://rustup.rs/
