# Const Closure

[![docs.rs](https://docs.rs/const_closure/badge.svg)](https://docs.rs/const_closure)
[![crates.io](https://img.shields.io/crates/v/const_closure.svg)](https://crates.io/crates/const_closure)
[![rustc](https://img.shields.io/badge/rustc-nightly-lightgrey)](https://doc.rust-lang.org/nightly/std/)

<!-- The rest of this section comes straight from the crate docs from the source. -->

# THIS CRATE IS BROKEN - Archive Note

The last tested working rust version is `1.71.0-nightly (nightly-2023-04-18)`.
Most const traits have been removed from the standard library.
Once they are back this crate might make a return if it is still needed.

## Description

This crate allows you to create types which represent closures in const contexts.

To do this simply create an instance of the provided closure helper: `ConstClosure`
with the associated `new` function.

A closure helper instance gets the data to capture as a tuple of same ownership (Owned, Mut or ref) and the implementation function to execute.

The implementation function must be a `const fn` that gets the captured state (a tuple of: owned values for `FnOnce`, `&mut` for `FnMut` and `&` for `Fn`)
and a tuple representing the arguments of the closure.

A closure helper instance returns the return value of the closure function on call.

## Example
```rust
#![feature(const_trait_impl)]
#![feature(const_mut_refs)] // Only required for the FnMut example

use const_closure::ConstClosure;

// FnOnce:
const _: () = {
  const fn imp((state,): (i32,), (arg,): (i32,)) -> i32 {
    state + arg
  }
  let cl = ConstClosure::new((5,), imp);
  assert!(7 == cl(2));
};

// FnMut:
const _: () = {
  const fn imp((state,): (&mut i32,), (arg,): (i32,)) -> i32 {
    *state += arg;
    *state
  }
  let mut i = 5;
  let mut cl = ConstClosure::new((&mut i,), imp);

  assert!(7 == cl(2));
  assert!(8 == cl(1));
};

// Fn:
const _: () = {
  const fn imp((state,): (&i32,), (arg,): (i32,)) -> i32 {
    *state + arg
  }
  let i = 5;
  let mut cl = ConstClosure::new((&i,), imp);
  assert!(7 == cl(2));
  assert!(8 == cl(3));
  assert!(6 == cl(1));
};
```

Note: The `const_closure` macro has been removed in favour of the new generic based approach.

Authors
-------

[raldone01](https://github.com/raldone01) and [onestacked](https://github.com/chriss0612) are the primary authors and maintainers of this library.

License
-------

This project is released under either:

- [MIT License](https://github.com/ink-feather-org/const_closure_rs/blob/master/LICENSE-MIT)
- [Apache License (Version 2.0)](https://github.com/ink-feather-org/const_closure_rs/blob/master/LICENSE-APACHE)

at your choosing.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
