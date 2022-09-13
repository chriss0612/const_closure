# Const Closure

[![Rust-CI](https://github.com/chriss0612/const_closure/actions/workflows/rust.yml/badge.svg)](https://github.com/chriss0612/const_closure/actions/workflows/rust.yml)
[![docs.rs](https://docs.rs/const_closure/badge.svg)](https://docs.rs/const_closure)
[![crates.io](https://img.shields.io/crates/v/const_closure.svg)](https://crates.io/crates/const_closure)
[![rustc](https://img.shields.io/badge/rustc-nightly-lightgrey)](https://doc.rust-lang.org/nightly/std/)

<!-- The rest of this section comes straight from the crate docs from the source. -->

This crate allows using macros to automatically create closure like types/values.

This works by implementing the Fn* traits for a struct containing all the values captured form the environment.

Because of restrictions of declarative macros the used syntax is less than ideal, but it is a lot better than manual implementations of the Fn* traits.

For details of the Syntax see: `const_closure`.

## Requirements

This crate requires a nightly compiler.

## Example
```rust
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(const_trait_impl)]
#![feature(const_mut_refs)]
#![feature(const_refs_to_cell)]
#![feature(const_ops)]
use const_closure::const_closure;

const FROM_CLOSURE: i32 = {
  let base = 5;
  let calc = const_closure!([base: i32] (offset: i32) -> i32 {
    base + offset
  });
  calc(-2)
};
assert_eq!(FROM_CLOSURE, 3)
```

Authors
-------

[raldone01](https://github.com/raldone01) and [onestacked](https://github.com/chriss0612) are the primary authors and maintainers of this library.

License
-------

This project is released under either:

- [MIT License](https://github.com/raldone01/trait_cast_rs/blob/main/LICENSE-MIT)
- [Apache License (Version 2.0)](https://github.com/raldone01/trait_cast_rs/blob/main/LICENSE-APACHE)

at your choosing.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.