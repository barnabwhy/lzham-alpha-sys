# lzham-alpha-sys

Low level Rust FFI bindings for [lzham alpha] generated using [`bindgen`].

You must have `cmake` and a C++ compiler to build this crate, as the [lzham] library is built along with the crate. The crate does not search for a prebuilt library.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
lzham-alpha-sys = "0.1.0"
```

## Features

The crate has the following three features:

- `generate_bindings`: Generates the bindings again (uses [`bindgen`])
- `static`: Links to the library statically
- `dynamic`: Links to the library dynamically

[lzham alpha]: https://github.com/richgel999/lzham_alpha
[lzham]: https://github.com/richgel999/lzham_alpha
[`bindgen`]: https://github.com/rust-lang/rust-bindgen

## License

lzham-alpha-sys is available under the MIT license. See [LICENSE](license) for more details.
