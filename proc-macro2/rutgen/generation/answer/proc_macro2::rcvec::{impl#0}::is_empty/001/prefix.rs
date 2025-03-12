// Answer 0

#[test]
fn test_is_empty_with_empty_vector() {
    let rc_vec: RcVec<i32> = RcVec {
        inner: Rc::new(vec![]),
    };
    rc_vec.is_empty();
}

#[test]
fn test_is_empty_with_single_element_vector() {
    let rc_vec: RcVec<i32> = RcVec {
        inner: Rc::new(vec![1]),
    };
    rc_vec.is_empty();
}

#[test]
fn test_is_empty_with_multiple_elements_vector() {
    let rc_vec: RcVec<i32> = RcVec {
        inner: Rc::new(vec![1, 2, 3]),
    };
    rc_vec.is_empty();
}

