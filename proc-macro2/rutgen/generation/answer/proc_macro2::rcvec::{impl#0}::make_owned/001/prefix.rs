// Answer 0

#[test]
fn test_make_owned_unique_mutable_reference() {
    let rc_vec = RcVec {
        inner: Rc::new(vec![1, 2, 3]),
    };
    let mut rc_vec_clone = rc_vec.clone(); // Create mutable copy
    let owned = Rc::get_mut(&mut rc_vec_clone.inner).unwrap(); // Ensure unique mutable reference
    let result = rc_vec_clone.make_owned(); // Call the function
}

#[test]
fn test_make_owned_unique_mutable_reference_string() {
    let rc_vec = RcVec {
        inner: Rc::new(vec!["a".to_string(), "b".to_string()]),
    };
    let mut rc_vec_clone = rc_vec.clone(); // Create mutable copy
    let owned = Rc::get_mut(&mut rc_vec_clone.inner).unwrap(); // Ensure unique mutable reference
    let result = rc_vec_clone.make_owned(); // Call the function
}

