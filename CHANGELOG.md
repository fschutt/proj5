# Changelog

## `0.1.6`

- Make `scoped_threadpool` optional if you don't need thread pools
- You can now build this library for WASM using `cargo build --target wasm32-unknown-unknown`

## `0.1.5`

- Updated to `dyn Trait` syntax to prepare for possible Rust 2018 breakage
- This version will only compile on Rust 1.27 or higher, use 0.1.4 for older compilers

## `0.1.4`

- Fixed large mathematical mistake in UTM projection