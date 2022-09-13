use core::ops::Add;

use super::*;

#[test]
fn once_closure() {
  let x = 5;
  let add1 = const_closure!(FnOnce [x: i32] () {
    x += 1;
    assert!(x == 6);
    ()
  });

  add1();

  assert_eq!(x, 5);

  let add = const_closure!(FnOnce [x: i32] (val1: i32) {
    x += val1;
    assert!(x == 10);
    ()
  });

  add(5);
  assert_eq!(x, 5);
}
#[test]
fn mut_closure() {
  let mut x = 5;

  let mut add1 = const_closure!(FnMut [x: i32] () -> () {
    *x += 1
  });

  add1();

  assert_eq!(x, 6);

  let mut add = const_closure!(FnMut [x: i32] (val1: i32) -> () {
    *x += val1
  });

  add(5);
  assert_eq!(x, 11);
}
#[test]
fn fn_closure() {
  let x = 5;

  let add1 = const_closure!([x: i32] () -> i32 {
    *x + 1
  });

  assert_eq!(add1(), 6);
  assert!(x == 5);

  let add = const_closure!([x: i32] (val1: i32) -> i32 {
    *x + val1
  });

  assert_eq!(add(5), 10);
  assert!(x == 5);
  let lt = const_closure!(for<T: PartialOrd> (val1: T, val2: T) -> bool {
    val1 < val2
  });
  assert_eq!(lt(5, 3), false)
}
#[test]
fn test_add() {
  const fn add<T, G: Add<T, Output = G>>(l: G, r: T) -> G {
    let cl = const_closure!(
        FnOnce for<T, G: Add<T, Output = G>> [l: G, r: T]
        () -> G {
      l + r
    });
    cl()
  }
  assert_eq!(add(1, 5), 6);
}
