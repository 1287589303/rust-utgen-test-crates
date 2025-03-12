// Answer 0

#[test]
fn test_iter_mut_left_empty() {
    let mut left: Either<Vec<u32>, &mut [u32]> = Left(vec![]);
    for _ in left.iter_mut() {}
}

#[test]
fn test_iter_mut_left_single_element() {
    let mut left: Either<Vec<u32>, &mut [u32]> = Left(vec![5]);
    for l in left.iter_mut() {
        *l *= 2;
    }
    assert_eq!(left, Left(vec![10]));
}

#[test]
fn test_iter_mut_left_multiple_elements() {
    let mut left: Either<Vec<u32>, &mut [u32]> = Left(vec![2, 4]);
    for l in left.iter_mut() {
        *l += 3;
    }
    assert_eq!(left, Left(vec![5, 7]));
}

#[test]
fn test_iter_mut_right_empty() {
    let mut inner: &mut [u32] = &mut [];
    let mut right: Either<Vec<u32>, _> = Right(inner);
    for _ in right.iter_mut() {}
}

#[test]
fn test_iter_mut_right_single_element() {
    let mut inner: &mut [u32] = &mut [1];
    let mut right: Either<Vec<u32>, _> = Right(inner);
    for r in right.iter_mut() {
        *r *= 5;
    }
    assert_eq!(inner, [5]);
}

#[test]
fn test_iter_mut_right_multiple_elements() {
    let mut inner: &mut [u32] = &mut [1, 2, 3];
    let mut right: Either<Vec<u32>, _> = Right(inner);
    for r in right.iter_mut() {
        *r += 2;
    }
    assert_eq!(inner, [3, 4, 5]);
}

