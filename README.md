# num-derive

[![crate](https://img.shields.io/crates/v/num-derive.svg)](https://crates.io/crates/num-derive)
[![documentation](https://docs.rs/num-derive/badge.svg)](https://docs.rs/num-derive)
[![Travis status](https://travis-ci.org/rust-num/num-derive.svg?branch=master)](https://travis-ci.org/rust-num/num-derive)

Procedural macros to derive numeric traits in Rust.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
num-derive= "0.1"
```

and this to your crate root:

```rust
#[macro_use]
extern crate num_derive;
```

## Compatibility

The `num-derive` crate is tested for rustc 1.15 and greater.
