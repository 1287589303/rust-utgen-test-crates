// Answer 0

#[test]
fn test_iter_empty() {
    let empty_vec: Rc<Vec<i32>> = Rc::new(Vec::new());
    let rc_vec = RcVec { inner: empty_vec };
    let mut iter = rc_vec.iter();
    let _ = iter.next(); // iterating over an empty RcVec
}

#[test]
fn test_iter_single_element() {
    let single_vec: Rc<Vec<i32>> = Rc::new(vec![42]);
    let rc_vec = RcVec { inner: single_vec };
    let mut iter = rc_vec.iter();
    assert_eq!(iter.next(), Some(&42)); // verifying the single element iteration
    let _ = iter.next(); // iterating to the end
}

#[test]
fn test_iter_multiple_elements() {
    let multiple_vec: Rc<Vec<i32>> = Rc::new(vec![1, 2, 3]);
    let rc_vec = RcVec { inner: multiple_vec };
    let mut iter = rc_vec.iter();
    assert_eq!(iter.next(), Some(&1)); // first element
    assert_eq!(iter.next(), Some(&2)); // second element
    assert_eq!(iter.next(), Some(&3)); // third element
    let _ = iter.next(); // iterating to the end
}

