#![no_std]
#![warn(clippy::pedantic, clippy::nursery)]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(const_trait_impl)]
#![feature(const_mut_refs)]
#![feature(const_refs_to_cell)]
#![feature(const_ptr_read)]
#![feature(const_option)]

/*!
# Const Closure
This crate allows using macros to automatically create closure like types/values.

This works by implementing the Fn* traits for a struct containing all the values captured form the environment.

Because of restrictions of declarative macros the used syntax is less than ideal, but it is a lot better than manual implementations of the Fn* traits.

For details of the Syntax see: `const_closure`.

# Example
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
*/

/// # Macro for creating const callable "closures"
///
/// The syntax is the following:
///
/// const_closure!(`fn_type` for<`generics`> [`captures`] (`args`) -> `return_type` {`closure_body`})
///
/// where:
/// * `fn_type` is one of the Fn* traits (Fn, FnMut, FnOnce).
/// * `generics` generics and bounds for them.
/// * `captures` the list of arguments that the closure captures with function argument like typing.
/// * `return_type` the return type of the closure.
/// * `closure_body` the actual implementation of the closure.
///
/// if `fn_type` is omitted it defaults to Fn.
///
/// if no `generics` are required, the for<> can be omitted.
///
/// if no `captures` are required, the [] can be omitted.
///
/// if `return_type` is (), -> () can be omitted.
///
/// #Advanced example:
/// ```
/// #![feature(unboxed_closures)]
/// #![feature(fn_traits)]
/// #![feature(const_trait_impl)]
/// #![feature(const_mut_refs)]
/// #![feature(const_refs_to_cell)]
/// use const_closure::const_closure;
/// use core::ops::Add;
///
/// const fn add<T, G: Add<T, Output = G>>(l: G, r: T) -> G {
///   let cl = const_closure!(FnOnce for<T, G: Add<T, Output = G>> [l: G, r: T] () -> G {
///     l + r
///   });
///   cl()
/// }
/// assert_eq!(add(1, 5), 6);
/// ```
#[cfg(doc)]
#[macro_export]
macro_rules! const_closure {
    ($($type:ident)? $(for<$($gen_name:ident $(: $($trait_bound:path)*)?),*>)? $this:ident $([$($cap_name:ident : $cap_ty:ty ),+ $(,)?])? ($($arg_name:ident: $arg_ty:ty),*) $(-> $ret_ty:ty)? {$($body:tt)*}) => { ... };
}

#[cfg(not(doc))]
#[macro_export]
#[allow(missing_docs)]
macro_rules! const_closure {
  // Syntax sugar
  (for $($end:tt)*) => {{
    const_closure!(Fn for$($end)*)
  }};
  ([$($mid:tt)*] $($end:tt)*) => {{
    const_closure!(Fn for<> [$($mid)*] $($end)*)
  }};
  (($($mid:tt)*) $($end:tt)*) => {{
    const_closure!(Fn for<> ($($mid)*) $($end)*)
  }};
  ($type:ident [$($mid:tt)*] $($end:tt)*) => {{
    const_closure!($type for<> [$($mid)*] $($end)*)
  }};
  ($type:ident for<$($gen_name:ident $(: $($trait_bound:path)*)?),*> ($($mid:tt)*) $($end:tt)*) => {{
    const_closure!($type for<$($gen_name $(: $($trait_bound)*)?),*> [] ($($mid)*) $($end)*)
  }};
  ($type:ident for<$($gen_name:ident $(: $($trait_bound:path)*)?),*> [$($cap_name:ident : $cap_ty:ty ),* $(,)?] ($($arg_name:ident: $arg_ty:ty),*) {$($body:tt)*}) => {{
    const_closure!($type for<$($gen_name $(: $($trait_bound)*)?),*> [$($cap_name : $cap_ty ),*] ($($arg_name: $arg_ty),*) -> () {$($body)*})
  }};
  // Actual Implementation
  (FnOnce for<$($gen_lt:lifetime,)* $($gen_name:ident $(:$($trait_bound:path)*)?),*>
  [$($cap_name:ident : $cap_ty:ty),* $(,)?]
  ($($arg_name:ident: $arg_ty:ty),*) -> $ret_ty:ty
  {$($body:tt)*}) => {{
    #[allow(non_snake_case)]
    struct Cl<$($gen_lt,)* $($gen_name $(: $($trait_bound)*)?),*>{
      _lt: ::core::marker::PhantomData<($(&$gen_lt (),)* $(*const $gen_name,)*)>,
      $($cap_name: $cap_ty),*
    }
    impl<$($gen_lt,)* $($gen_name $(: $(~const $trait_bound + ) + ~const ::core::marker::Destruct)?),*> const FnOnce<($($arg_ty,)*)> for Cl<$($gen_name),*>
      where Self: ~const ::core::marker::Destruct {
      type Output = $ret_ty;

      #[allow(unused_parens)]
      extern "rust-call" fn call_once(self, ($($arg_name,)*): ($($arg_ty,)*)) -> Self::Output {
        $(
          #[allow(unused_mut)]
          #[allow(unused_variables)]
          let mut $cap_name = self.$cap_name;
        )*
        $($body)*
      }
    }
    Cl {
      _lt: ::core::marker::PhantomData,
      $($cap_name),*
    }
  }};
  (FnMut for<$($gen_lt:lifetime,)* $($gen_name:ident $(:$($trait_bound:path)*)?),*>
    [$($cap_name:ident : $cap_ty:ty),* $(,)?]
    ($($arg_name:ident: $arg_ty:ty),*) -> $ret_ty:ty
    {$($body:tt)*}) => {{
    #[allow(non_snake_case)]
    struct Cl<'a, $($gen_name $(: $($trait_bound)*)?),*> {
      _lt: ::core::marker::PhantomData<(&'a mut u8, $(&$gen_lt (),)* $(*const $gen_name),*)>,
      $($cap_name: Option<&'a mut $cap_ty>,)*
    }
    impl<'a, $($gen_lt,)* $($gen_name $(: $(~const $trait_bound + )* ~const ::core::marker::Destruct)?),*> const FnOnce<($($arg_ty,)*)> for Cl<'a, $($gen_name),*> {
      type Output = $ret_ty;

      #[allow(unused_parens)]
      extern "rust-call" fn call_once(mut self, args: ($($arg_ty,)*)) -> Self::Output {
        self.call_mut(args)
      }
    }

    impl<'a, $($gen_lt,)* $($gen_name $(: $(~const $trait_bound + )* ~const ::core::marker::Destruct)?),*> const FnMut<($($arg_ty,)*)> for Cl<'a, $($gen_name),*> {

      #[allow(unused_parens)]
      extern "rust-call" fn call_mut(&mut self, ($($arg_name,)*): ($($arg_ty,)*)) -> Self::Output {
        // Hack to extract the references from Self
        $(let $cap_name = self.$cap_name.take().unwrap();)*
        let ret = { $($body)* };
        $(self.$cap_name = Some($cap_name);)*
        ret
      }
    }
    Cl {
      _lt: ::core::marker::PhantomData,
      $($cap_name: Some(&mut $cap_name),)*
    }
  }};
  (Fn for<$($gen_lt:lifetime,)* $($gen_name:ident $(:$($trait_bound:path)*)?),*>
  [$($cap_name:ident : $cap_ty:ty),* $(,)?]
  ($($arg_name:ident: $arg_ty:ty),*) -> $ret_ty:ty
  {$($body:tt)*}) => {{
    #[allow(non_snake_case)]
    struct Cl<'a, $($gen_name $(: $($trait_bound)*)?),*> {
      _lt: ::core::marker::PhantomData<(&'a mut u8, $(&mut $gen_lt (),)* $(*const $gen_name),*)>,
      $($gen_name: ::core::marker::PhantomData<*const $gen_name>,)*
      $($cap_name: &'a $cap_ty,)*
    }
    impl<'a, $($gen_lt,)* $($gen_name $(: $(~const $trait_bound + )+ ~const ::core::marker::Destruct)?),*> const FnOnce<($($arg_ty,)*)> for Cl<'a, $($gen_name),*> {
      type Output = $ret_ty;

      extern "rust-call" fn call_once(mut self, args: ($($arg_ty,)*)) -> Self::Output {
        self.call_mut(args)
      }
    }
    impl<'a, $($gen_lt,)* $($gen_name $(: $(~const $trait_bound + )+ ~const ::core::marker::Destruct)?),*> const FnMut<($($arg_ty,)*)> for Cl<'a, $($gen_name),*> {
      extern "rust-call" fn call_mut(&mut self, args: ($($arg_ty,)*)) -> Self::Output {
        self.call(args)
      }
    }
    impl<'a, $($gen_lt,)* $($gen_name $(: $(~const $trait_bound + )+ ~const ::core::marker::Destruct)?),*> const Fn<($($arg_ty,)*)> for Cl<'a, $($gen_name),*> {

      #[allow(unused_parens)]
      extern "rust-call" fn call(&self, ($($arg_name,)*): ($($arg_ty,)*)) -> Self::Output {
        $(
          #[allow(unused_variables)]
          let $cap_name = self.$cap_name;
        )*
        $($body)*
      }
    }
    Cl {
      _lt: ::core::marker::PhantomData,
      $($cap_name: &$cap_name,)*
    }
  }};
}

#[cfg(test)]
mod tests;
