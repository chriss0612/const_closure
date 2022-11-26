use core::marker::{Destruct, Tuple};

/// Struct representing a closure with owned data.
///
/// Example:
/// ```rust
/// use const_closure::ConstClosure;
/// const fn imp((state,): (i32,), (arg,): (i32,)) -> i32 {
///     state + arg
/// }
/// let i = 5;
/// let cl = ConstClosure::new((i,), imp);
///
/// assert!(7 == cl(2));
/// ```
pub struct ConstClosure<CapturedData, Function> {
  data: CapturedData,
  func: Function,
}
impl<CapturedData: Tuple, Function> ConstClosure<CapturedData, Function> {
  /// Function for creating a new closure.
  ///
  /// `data` is the owned data that is captured from the environment, must be a tuple.
  ///
  /// `func` is the function of the closure, it gets the data and a tuple of the arguments closure
  ///   and return the return value of the closure.
  pub const fn new(data: CapturedData, func: Function) -> Self {
    Self { data, func }
  }
}

macro_rules! impl_const_closure {
  ($T:ident) => {
    impl_const_closure!(@impl $T);
};
  ($T:ident $($U:ident)+) => {
    impl_const_closure!($($U)+);
    impl_const_closure!(@impl $T $( $U )+);
  };
  (@impl $($var:ident)+) => {
    impl<$($var,)+ ClosureArguments: Tuple, Function, ClosureReturnValue> const
      FnOnce<ClosureArguments> for ConstClosure<($($var,)+), Function>
    where
      Function:
        ~const FnOnce(($($var,)+), ClosureArguments) -> ClosureReturnValue + ~const Destruct,
        Self: ~const Destruct
    {
      type Output = ClosureReturnValue;

      extern "rust-call" fn call_once(self, args: ClosureArguments) -> Self::Output {
        (self.func)(self.data, args)
      }
    }
    impl<'a, $($var,)+ ClosureArguments: Tuple, Function, ClosureReturnValue> const
      FnMut<ClosureArguments> for ConstClosure<($(&'a mut $var,)+), Function>
    where
      Function:
        ~const FnMut(($(&mut $var,)+), ClosureArguments) -> ClosureReturnValue + ~const Destruct,
    {
      extern "rust-call" fn call_mut(&mut self, args: ClosureArguments) -> Self::Output {
        #[allow(non_snake_case)]
        let ($($var,)*) = &mut self.data;
        (self.func)(($($var,)*), args)
      }
    }
    impl<'a, $($var,)+ ClosureArguments: Tuple, Function, ClosureReturnValue> const
    FnMut<ClosureArguments> for ConstClosure<($(&'a $var,)+), Function>
    where
      Function:
        ~const FnMut(($(&$var,)+), ClosureArguments) -> ClosureReturnValue + ~const Destruct,
    {
      extern "rust-call" fn call_mut(&mut self, args: ClosureArguments) -> Self::Output {
        (self.func)(self.data, args)
      }
    }
    impl<'a, $($var,)+ ClosureArguments: Tuple, Function, ClosureReturnValue> const
      Fn<ClosureArguments> for ConstClosure<($(&'a $var,)+), Function>
    where
      Function:
        ~const Fn(($(&$var,)+), ClosureArguments) -> ClosureReturnValue + ~const Destruct,
    {
      extern "rust-call" fn call(&self, args: ClosureArguments) -> Self::Output {
        (self.func)(self.data, args)
      }
    }
  };
}

impl_const_closure!(A B C D E F G H I J K L);
