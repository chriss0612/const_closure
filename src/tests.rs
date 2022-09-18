use crate::closure_type::{ConstFnClosure, ConstFnMutClosure, ConstFnOnceClosure};

#[test]
const fn test_fn_once_closure_struct() {
  const fn imp(state: i32, (arg,): (i32,)) -> i32 {
    state + arg
  }
  let i = 5;
  let cl = ConstFnOnceClosure::new(i, imp);

  assert!(7 == cl(2));
}

#[test]
const fn test_fn_mut_closure_struct() {
  const fn imp(state: &mut i32, (arg,): (i32,)) -> i32 {
    *state += arg;
    *state
  }
  let mut i = 5;
  let mut cl = ConstFnMutClosure::new(&mut i, imp);

  assert!(7 == cl(2));
  assert!(8 == cl(1));
}

#[test]
const fn test_fn_closure_struct() {
  const fn imp(state: &i32, (arg,): (i32,)) -> i32 {
    *state + arg
  }
  let i = 5;
  let cl = ConstFnClosure::new(&i, imp);

  assert!(7 == cl(2));
  assert!(6 == cl(1));
}
