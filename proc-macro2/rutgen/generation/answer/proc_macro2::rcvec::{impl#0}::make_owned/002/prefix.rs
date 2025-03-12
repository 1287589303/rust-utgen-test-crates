// Answer 0

#[test]
fn test_make_owned_unique_rc() {
    let unique_vec = Rc::new(vec![1, 2, 3]);
    let rc_vec = RcVec { inner: unique_vec };

    let builder = rc_vec.make_owned();

    let expected_inner = vec![1, 2, 3];
    let actual_inner = builder.inner;

    // Following lines are just calls without assertions
    let _ = expected_inner; 
    let _ = actual_inner;
}

