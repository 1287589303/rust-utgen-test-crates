// Answer 0

#[test]
fn test_extend_with_non_empty_iterator() {
    let mut vec = vec![1, 2, 3];
    let mut rc_vec_mut = RcVecMut { inner: &mut vec };
    rc_vec_mut.extend(vec![4, 5, 6]);
}

#[test]
fn test_extend_with_empty_iterator() {
    let mut vec = vec![1, 2, 3];
    let mut rc_vec_mut = RcVecMut { inner: &mut vec };
    rc_vec_mut.extend(vec![]);
}

#[test]
fn test_extend_with_single_element_iterator() {
    let mut vec = vec![1, 2, 3];
    let mut rc_vec_mut = RcVecMut { inner: &mut vec };
    rc_vec_mut.extend(vec![4]);
}

#[test]
fn test_extend_with_iterator_of_different_type() {
    let mut vec = vec![1, 2, 3];
    let mut rc_vec_mut = RcVecMut { inner: &mut vec };
    rc_vec_mut.extend(vec![7.0, 8.0]); // This will fail at compile-time due to type mismatch as Vec<T> is inferred as Vec<i32> from above.
}

