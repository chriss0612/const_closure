#![no_std]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(const_trait_impl)]
#![feature(const_mut_refs)]
#![feature(const_refs_to_cell)]

#[macro_export]
macro_rules! const_closure {
  // Syntax sugar
  (for $($end:tt)*) => {{
    const_closure!(Fn for$($end)*)
  }};
  ($this:ident [$($mid:tt)*] $($end:tt)*) => {{
    const_closure!(Fn for<> $this [$($mid)*] $($end)*)
  }};
  ($this:ident ($($mid:tt)*) $($end:tt)*) => {{
    const_closure!(Fn for<> $this ($($mid)*) $($end)*)
  }};
  ($type:ident $this:ident [$($mid:tt)*] $($end:tt)*) => {{
    const_closure!($type for<> $this [$($mid)*] $($end)*)
  }};
  ($type:ident for<$($gen_name:ident $(: $($trait_bound:path)*)?),*> $this:ident ($($mid:tt)*) $($end:tt)*) => {{
    const_closure!($type for<$($gen_name $(: $($trait_bound)*)?),*> $this [] ($($mid)*) $($end)*)
  }};
  ($type:ident for<$($gen_name:ident $(: $($trait_bound:path)*)?),*> $this:ident [$($cap_name:ident : $cap_ty:ty ),* $(,)?] ($($arg_name:ident: $arg_ty:ty),*) {$($body:tt)*}) => {{
    const_closure!($type for<$($gen_name $(: $($trait_bound)*)?),*> $this [$($cap_name : $cap_ty ),*] ($($arg_name: $arg_ty),*) -> () {$($body)*})
  }};
  // Actual Implementation
  (FnOnce for<$($gen_name:ident $(: $($trait_bound:path)*)?),*> $this:ident [$($cap_name:ident : $cap_ty:ty ),+ $(,)?] ($($arg_name:ident: $arg_ty:ty),*) -> $ret_ty:ty {$($body:tt)*}) => {{
    #[allow(non_snake_case)]
    struct Cl<$($gen_name $(: $($trait_bound)*)?),*> {
      $($gen_name: ::core::marker::PhantomData<*const $gen_name>),*
      $($cap_name: $cap_ty),*
    }
    impl<$($gen_name $(: $(~const $trait_bound + )+ ~const ::core::marker::Destruct)?),*> const FnOnce<($($arg_ty,)*)> for Cl<$($gen_name),*> {
      type Output = $ret_ty;

      #[allow(unused_parens)]
      extern "rust-call" fn call_once(self, ($($arg_name,)*): ($($arg_ty,)*)) -> Self::Output {
        #[allow(unused_mut)]
        #[allow(unused_variables)]
        let mut $this = self;
        $($body)*
      }
    }
    Cl {
      $($gen_name: ::core::marker::PhantomData),*
      $($cap_name: $cap_name),*
    }
  }};
  (FnMut for<$($gen_name:ident $(: $($trait_bound:path)*)?),*> $this:ident [$($cap_name:ident : $cap_ty:ty),* $(,)?] ($($arg_name:ident: $arg_ty:ty),*) -> $ret_ty:ty {$($body:tt)*}) => {{
    #[allow(non_snake_case)]
    struct Cl<'a, $($gen_name $(: $($trait_bound)*)?),*> {
      _lt: ::core::marker::PhantomData<&'a mut u8>,
      $($gen_name: ::core::marker::PhantomData<*const $gen_name>),*
      $($cap_name: &'a mut $cap_ty),*
    }
    impl<'a, $($gen_name $(: $(~const $trait_bound + )+ ~const ::core::marker::Destruct)?),*> const FnOnce<($($arg_ty,)*)> for Cl<'a, $($gen_name),*> {
      type Output = $ret_ty;

      #[allow(unused_parens)]
      extern "rust-call" fn call_once(mut self, args: ($($arg_ty,)*)) -> Self::Output {
        self.call_mut(args)
      }
    }
    impl<'a, $($gen_name $(: $(~const $trait_bound + )+ ~const ::core::marker::Destruct)?),*> const FnMut<($($arg_ty,)*)> for Cl<'a, $($gen_name),*> {

      #[allow(unused_parens)]
      extern "rust-call" fn call_mut(&mut self, ($($arg_name,)*): ($($arg_ty,)*)) -> Self::Output {
        #[allow(unused_variables)]
        let $this = self;
        $($body)*
      }
    }
    Cl {
      _lt: ::core::marker::PhantomData,
      $($gen_name: ::core::marker::PhantomData),*
      $($cap_name: &mut $cap_name),*
    }
  }};
  (Fn for<$($gen_name:ident $(: $($trait_bound:path)*)?),*> $this:ident [$($cap_name:ident : $cap_ty:ty ),* $(,)?] ($($arg_name:ident: $arg_ty:ty),*) -> $ret_ty:ty {$($body:tt)*}) => {{
    #[allow(non_snake_case)]
    struct Cl<'a, $($gen_name $(: $($trait_bound)*)?),*> {
      _lt: ::core::marker::PhantomData<&'a u8>,
      $($gen_name: ::core::marker::PhantomData<*const $gen_name>),*
      $($cap_name: &'a $cap_ty),*
    }
    impl<'a, $($gen_name $(: $(~const $trait_bound + )+ ~const ::core::marker::Destruct)?),*> const FnOnce<($($arg_ty,)*)> for Cl<'a, $($gen_name),*> {
      type Output = $ret_ty;

      extern "rust-call" fn call_once(mut self, args: ($($arg_ty,)*)) -> Self::Output {
        self.call_mut(args)
      }
    }
    impl<'a, $($gen_name $(: $(~const $trait_bound + )+ ~const ::core::marker::Destruct)?),*> const FnMut<($($arg_ty,)*)> for Cl<'a, $($gen_name),*> {
      extern "rust-call" fn call_mut(&mut self, args: ($($arg_ty,)*)) -> Self::Output {
        self.call(args)
      }
    }
    impl<'a, $($gen_name $(: $(~const $trait_bound + )+ ~const ::core::marker::Destruct)?),*> const Fn<($($arg_ty,)*)> for Cl<'a, $($gen_name),*> {

      #[allow(unused_parens)]
      extern "rust-call" fn call(&self, ($($arg_name,)*): ($($arg_ty,)*)) -> Self::Output {
        #[allow(unused_variables)]
        let $this = self;
        $($body)*
      }
    }
    Cl {
      _lt: ::core::marker::PhantomData,
      $($gen_name: ::core::marker::PhantomData),*
      $($cap_name: &mut $cap_name),*
    }
  }};
}

#[cfg(test)]
mod tests;
