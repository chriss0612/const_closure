# Const Closure

[![Rust-CI](https://github.com/chriss0612/const_closure/actions/workflows/rust.yml/badge.svg)](https://github.com/chriss0612/const_closure/actions/workflows/rust.yml)
[![docs.rs](https://docs.rs/const_closure/badge.svg)](https://docs.rs/const_closure)
[![crates.io](https://img.shields.io/crates/v/const_closure.svg)](https://crates.io/crates/const_closure)
[![rustc](https://img.shields.io/badge/rustc-nightly-lightgrey)](https://doc.rust-lang.org/nightly/std/)

<!-- The rest of this section comes straight from the crate docs from the source. -->

This crate allows to create types for representing closures in const contexts.

To do this simply create a instance of one of the Const{Fn, FnMut, FnOnce}Closure
with the associated new function.

This new function gets a the data to be captured (owned for FnOnce, &mut for FnMut and & for Fn)
and the the function to execute.

This function must be a const fn that gets the captured state (owned for FnOnce, &mut for FnMut and & for Fn)
and a tuple representing the arguments of the closure.

The Closure returns the return value of that function.

If you were looking for the const_closure macro, this was removed in version 2.0 in favor of the new generic based approach
  as this is a lot cleaner and also more versatile.

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
*/

Authors
-------

[raldone01](https://github.com/raldone01) and [onestacked](https://github.com/chriss0612) are the primary authors and maintainers of this library.

License
-------

This project is released under either:

- [MIT License](https://github.com/chriss0612/const_closure/blob/master/LICENSE-MIT)
- [Apache License (Version 2.0)](https://github.com/chriss0612/const_closure/blob/master/LICENSE-APACHE)

at your choosing.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.