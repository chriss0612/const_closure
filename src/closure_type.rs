use core::marker::{Destruct, PhantomData};

/// Struct representing a closure with owned data.
///
/// Example:
/// ```rust
/// use const_closure::ConstFnOnceClosure;
/// const fn imp(state: i32, (arg,): (i32,)) -> i32 {
///     state + arg
/// }
/// let i = 5;
/// let cl = ConstFnOnceClosure::new(i, imp);
///
/// assert!(7 == cl(2));
/// ```
pub struct ConstFnOnceClosure<CapturedData, ClosureArguments, Function> {
  data: CapturedData,
  func: Function,
  _ph: PhantomData<ClosureArguments>,
}
impl<CapturedData, ClosureArguments, Function>
  ConstFnOnceClosure<CapturedData, ClosureArguments, Function>
{
  /// Function for creating a new closure.
  ///
  /// `data` is the owned data that is captured from the environment (this data must be `~const Destruct`).
  ///
  /// `func` is the function of the closure, it gets the data and a tuple of the arguments closure
  ///   and return the return value of the closure.
  pub const fn new<ClosureReturnValue>(data: CapturedData, func: Function) -> Self
  where
    CapturedData: ~const Destruct,
    Function: ~const Fn(CapturedData, ClosureArguments) -> ClosureReturnValue + ~const Destruct,
  {
    Self {
      data,
      func,
      _ph: PhantomData,
    }
  }
}
impl<CapturedData, ClosureArguments, Function, ClosureReturnValue> const FnOnce<ClosureArguments>
  for ConstFnOnceClosure<CapturedData, ClosureArguments, Function>
where
  CapturedData: ~const Destruct,
  Function: ~const Fn(CapturedData, ClosureArguments) -> ClosureReturnValue + ~const Destruct,
{
  type Output = ClosureReturnValue;

  extern "rust-call" fn call_once(self, args: ClosureArguments) -> Self::Output {
    (self.func)(self.data, args)
  }
}

/// Struct representing a closure with mutably borrowed data.
///
/// Example:
/// ```rust
/// #![feature(const_mut_refs)]
/// use const_closure::ConstFnMutClosure;
/// const fn imp(state: &mut i32, (arg,): (i32,)) -> i32 {
///   *state += arg;
///   *state
/// }
/// let mut i = 5;
/// let mut cl = ConstFnMutClosure::new(&mut i, imp);
///
/// assert!(7 == cl(2));
/// assert!(8 == cl(1));
/// ```
pub struct ConstFnMutClosure<'a, CapturedData: ?Sized, ClosureArguments, Function> {
  data: &'a mut CapturedData,
  func: Function,
  _ph: PhantomData<ClosureArguments>,
}
impl<'a, CapturedData: ?Sized, ClosureArguments, Function>
  ConstFnMutClosure<'a, CapturedData, ClosureArguments, Function>
{
  /// Function for creating a new closure.
  ///
  /// `data` is the a mutable borrow of data that is captured from the environment.
  ///
  /// `func` is the function of the closure, it gets the data and a tuple of the arguments closure
  ///   and return the return value of the closure.
  pub const fn new<ClosureReturnValue>(data: &'a mut CapturedData, func: Function) -> Self
  where
    Function:
      ~const Fn(&mut CapturedData, ClosureArguments) -> ClosureReturnValue + ~const Destruct,
  {
    Self {
      data,
      func,
      _ph: PhantomData,
    }
  }
}
impl<'a, CapturedData: ?Sized, ClosureArguments, Function, ClosureReturnValue> const
  FnOnce<ClosureArguments> for ConstFnMutClosure<'a, CapturedData, ClosureArguments, Function>
where
  Function: ~const Fn(&mut CapturedData, ClosureArguments) -> ClosureReturnValue + ~const Destruct,
{
  type Output = ClosureReturnValue;

  extern "rust-call" fn call_once(mut self, args: ClosureArguments) -> Self::Output {
    self.call_mut(args)
  }
}
impl<'a, CapturedData: ?Sized, ClosureArguments, Function, ClosureReturnValue> const
  FnMut<ClosureArguments> for ConstFnMutClosure<'a, CapturedData, ClosureArguments, Function>
where
  Function: ~const Fn(&mut CapturedData, ClosureArguments) -> ClosureReturnValue + ~const Destruct,
{
  extern "rust-call" fn call_mut(&mut self, args: ClosureArguments) -> Self::Output {
    (self.func)(self.data, args)
  }
}

/// Struct representing a closure with borrowed data.
///
/// Example:
/// ```rust
/// use const_closure::ConstFnClosure;
///
/// const fn imp(state: &i32, (arg,): (i32,)) -> i32 {
///     *state + arg
/// }
/// let i = 5;
/// let cl = ConstFnClosure::new(&i, imp);
///
/// assert!(7 == cl(2));
/// assert!(6 == cl(1));
/// ```
pub struct ConstFnClosure<'a, CapturedData: ?Sized, ClosureArguments, Function> {
  data: &'a CapturedData,
  func: Function,
  _ph: PhantomData<ClosureArguments>,
}
impl<'a, CapturedData: ?Sized, ClosureArguments, Function>
  ConstFnClosure<'a, CapturedData, ClosureArguments, Function>
{
  /// Function for creating a new closure.
  ///
  /// `data` is the a mutable borrow of data that is captured from the environment.
  ///
  /// `func` is the function of the closure, it gets the data and a tuple of the arguments closure
  ///   and return the return value of the closure.
  pub const fn new<ClosureReturnValue>(data: &'a CapturedData, func: Function) -> Self
  where
    Function: ~const Fn(&CapturedData, ClosureArguments) -> ClosureReturnValue + ~const Destruct,
  {
    Self {
      data,
      func,
      _ph: PhantomData,
    }
  }
}
impl<'a, CapturedData: ?Sized, Function, ClosureArguments, ClosureReturnValue> const
  FnOnce<ClosureArguments> for ConstFnClosure<'a, CapturedData, ClosureArguments, Function>
where
  Function: ~const Fn(&CapturedData, ClosureArguments) -> ClosureReturnValue + ~const Destruct,
{
  type Output = ClosureReturnValue;

  extern "rust-call" fn call_once(mut self, args: ClosureArguments) -> Self::Output {
    self.call_mut(args)
  }
}
impl<'a, CapturedData: ?Sized, Function, ClosureArguments, ClosureReturnValue> const
  FnMut<ClosureArguments> for ConstFnClosure<'a, CapturedData, ClosureArguments, Function>
where
  Function: ~const Fn(&CapturedData, ClosureArguments) -> ClosureReturnValue + ~const Destruct,
{
  extern "rust-call" fn call_mut(&mut self, args: ClosureArguments) -> Self::Output {
    self.call(args)
  }
}
impl<
    'a,
    CapturedData: ?Sized,
    Function: ~const Fn(&CapturedData, ClosureArguments) -> ClosureReturnValue + ~const Destruct,
    ClosureArguments,
    ClosureReturnValue,
  > const Fn<ClosureArguments> for ConstFnClosure<'a, CapturedData, ClosureArguments, Function>
{
  extern "rust-call" fn call(&self, args: ClosureArguments) -> Self::Output {
    (self.func)(self.data, args)
  }
}
