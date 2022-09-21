use core::{cmp::Ordering, marker::Destruct};

use crate::ConstFnMutClosure;

#[test]
fn test1() {
  const fn func(a: &i32) -> i32 {
    *a + 2
  }
  let mut f = func;
  const fn consume<T, F>(_: F)
  where
    F: ~const FnMut(&T, &T) -> Option<Ordering> + ~const Destruct,
  {
  }

  const fn imp<T: ~const Destruct, F, K: ~const PartialOrd + ~const Destruct>(
    f: &mut F,
    (a, b): (&T, &T),
  ) -> Option<Ordering>
  where
    F: ~const Fn(&T) -> K + ~const Destruct,
  {
    f(a).partial_cmp(&f(b))
  }
  let cl = ConstFnMutClosure::new(&mut f, imp); //|f, (a, b): (&i32, &i32)| None);

  consume(cl)
}

#[test]
const fn test2() {
  struct Func<T>(T);

  impl<A, T: ~const FnOnce<A> + ~const Destruct> const FnOnce<A> for Func<T> {
    type Output = T::Output;

    extern "rust-call" fn call_once(self, args: A) -> Self::Output {
      self.0.call_once(args)
    }
  }

  const fn test(_a: &i32) -> Option<Ordering> {
    None
  }

  let cl = Func(test);

  const fn consume<T, F>(_: F)
  where
    F: ~const FnOnce(&T) -> Option<Ordering> + ~const Destruct,
  {
  }
  consume(cl);
}
#[test]
const fn test3() {
  const fn imp<T, F, K: ~const PartialOrd + ~const Destruct>(
    f: &mut F,
    (a, b): (&T, &T),
  ) -> Option<core::cmp::Ordering>
  where
    F: ~const FnMut(&T) -> K + ~const Destruct,
  {
    f(a).partial_cmp(&f(b))
  }
  const fn trans<T>(_: &T) -> i32 {
    3
  }
  let mut tr = trans::<i32>;
  let mut cl = ConstFnMutClosure::new(&mut tr, imp);

  const fn consume<T, F>(_: F)
  where
    F: ~const FnMut(&T, &T) -> Option<Ordering> + ~const Destruct,
  {
  }
  consume(&mut cl);

  const fn const_is_sorted_by<T, F>(_: F) -> bool
  where
    F: ~const FnMut(&T, &T) -> Option<Ordering> + ~const Destruct,
  {
    true
  }
  const_is_sorted_by(&mut cl);

  const fn imp2<T, F, K: ~const PartialOrd + ~const Destruct>(
    f: &mut F,
    (a, b): (&T, &T),
  ) -> Option<core::cmp::Ordering>
  where
    F: ~const Fn(&T) -> K + ~const Destruct,
  {
    f(a).partial_cmp(&f(b))
  }
  const fn testx<T>(_: &T) -> i32 {
    5
  }
  let mut f = testx::<i32>;
  const_is_sorted_by(ConstFnMutClosure::new(&mut f, imp2));
}
