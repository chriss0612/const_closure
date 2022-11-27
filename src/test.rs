use core::{
  cmp::Ordering,
  marker::{Destruct, Tuple},
};

use crate::ConstClosure;

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
    (f,): (&mut F,),
    (a, b): (&T, &T),
  ) -> Option<Ordering>
  where
    F: ~const Fn(&T) -> K + ~const Destruct,
  {
    f(a).partial_cmp(&f(b))
  }
  let cl = ConstClosure::new((&mut f,), imp);

  consume(cl);
}

#[test]
const fn test2() {
  struct Func<T>(T);

  impl<A: Tuple, T: ~const FnOnce<A> + ~const Destruct> const FnOnce<A> for Func<T> {
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
    (f,): (&mut F,),
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
  let mut cl = ConstClosure::new((&mut tr,), imp);

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
    (f,): (&mut F,),
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
  const_is_sorted_by(ConstClosure::new((&mut f,), imp2));
}

#[test]
fn test_multiple() {
  const fn consumer<F: ~const FnOnce() -> i32>(f: F) -> i32 {
    f()
  }
  const fn user(x: i32, y: i32) -> i32 {
    const fn imp((x, y): (i32, i32), _: ()) -> i32 {
      x + y
    }
    consumer(ConstClosure::new((x, y), imp))
  }

  const _: () = {
    let x = 1;
    let y = 4;
    assert!(user(x, y) == 5);
  };
  let x = 1;
  let y = 7;

  assert_eq!(user(x, y), 8);
}

#[test]
fn test_multiple_mut() {
  const fn consumer<F: ~const FnMut(i32) + ~const Destruct>(mut f: F) {
    f(5);
  }
  const fn user(x: &mut i32, y: &mut i32) {
    const fn imp((x, y): (&mut i32, &mut i32), (val,): (i32,)) {
      *x += val;
      *y += val;
    }
    consumer(ConstClosure::new((x, y), imp));
  }

  const _: () = {
    let mut x = 0;
    let mut y = 4;
    user(&mut x, &mut y);

    assert!(x == 5);
    assert!(y == 9);
  };
  let mut x = 1;
  let mut y = 7;

  user(&mut x, &mut y);

  assert_eq!(x, 6);
  assert_eq!(y, 12);
}

#[test]
fn test_multiple_ref() {
  const fn consumer<F: ~const Fn() -> i32 + ~const Destruct>(f: F) -> i32 {
    f()
  }
  const fn user(x: &i32, y: &i32) -> i32 {
    const fn imp((x, y): (&i32, &i32), _: ()) -> i32 {
      *x + *y
    }
    consumer(ConstClosure::new((x, y), imp))
  }

  const _: () = {
    let x = 1;
    let y = 4;
    assert!(user(&x, &y) == 5);
  };
  let x = 1;
  let y = 7;

  assert_eq!(user(&x, &y), 8);
}
