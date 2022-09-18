#![no_std]
#![warn(clippy::pedantic, clippy::nursery)]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(const_trait_impl)]
#![feature(const_mut_refs)]
#![feature(const_refs_to_cell)]
/*!
# Const Closure
This crate allows to create types for representing closures in const contexts.

To do this simply create a instance of one of the Const{Fn, FnMut, FnOnce}Closure
with the associated new function.

This new function gets a the data to be captured (owned for FnOnce, &mut for FnMut and & for Fn)
and the the function to execute.

This function must be a const fn that gets the captured state (owned for FnOnce, &mut for FnMut and & for Fn)
and a tuple representing the arguments of the closure.

The Closure returns the return value of that function.

If you were looking for the const_closure macro, this was removed in favor of the new generic based approach
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
```
*/

mod closure_type;
pub use closure_type::{ConstFnClosure, ConstFnMutClosure, ConstFnOnceClosure};
