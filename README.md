# Const Closure

[![Daily-Nightly](https://github.com/ink-feather-org/const_closure/actions/workflows/rust_daily_nightly_check.yml/badge.svg)](https://github.com/ink-feather-org/const_closure/actions/workflows/rust_daily_nightly_check.yml)
[![Rust-Main-CI](https://github.com/ink-feather-org/const_closure/actions/workflows/rust_main.yml/badge.svg)](https://github.com/ink-feather-org/const_closure/actions/workflows/rust_main.yml)
[![docs.rs](https://docs.rs/const_closure/badge.svg)](https://docs.rs/const_closure)
[![crates.io](https://img.shields.io/crates/v/const_closure.svg)](https://crates.io/crates/const_closure)
[![rustc](https://img.shields.io/badge/rustc-nightly-lightgrey)](https://doc.rust-lang.org/nightly/std/)

<!-- The rest of this section comes straight from the crate docs from the source. -->

This crate allows you to create types which represent closures in const contexts.

To do this simply create an instance of one of the provided closure helpers: `Const{Fn, FnMut, FnOnce}Closure`
with their associated `new` function.

A closure helper instance gets the data to capture (owned for `FnOnce`, `&mut` for `FnMut` and `&` for `Fn`)
and the closure function to execute.

The closure function must be a `const fn` that gets the captured state (owned for `FnOnce`, `&mut` for `FnMut` and `&` for `Fn`)
and a tuple representing the arguments of the closure.

A closure helper instance returns the return value of the closure function.

## Example
```rust
#![feature(const_mut_refs)]
use const_closure::ConstFnMutClosure;
const fn imp(state: &mut i32, (arg,): (i32,)) -> i32 {
  *state += arg;
  *state
}
let mut i = 5;
let mut cl = ConstFnMutClosure::new(&mut i, imp);

assert!(7 == cl(2));
assert!(8 == cl(1));
```

Note: The `const_closure` macro has been removed in favour of the new generic based approach.

Authors
-------

[raldone01](https://github.com/raldone01) and [onestacked](https://github.com/chriss0612) are the primary authors and maintainers of this library.

License
-------

This project is released under either:

- [MIT License](https://github.com/ink-feather-org/const_closure/blob/master/LICENSE-MIT)
- [Apache License (Version 2.0)](https://github.com/ink-feather-org/const_closure/blob/master/LICENSE-APACHE)

at your choosing.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
