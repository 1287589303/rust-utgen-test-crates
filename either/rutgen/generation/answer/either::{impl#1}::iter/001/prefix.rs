// Answer 0

#[test]
fn test_iter_right_with_vector() {
    let right: Either<Vec<u32>, _> = Right(vec![1, 2, 3]);
    let mut all = vec![0];
    all.extend(right.iter());
}

#[test]
fn test_iter_right_with_slice() {
    let left: Either<_, &[u32]> = Left(vec![4, 5]);
    let right: Either<Vec<u32>, _> = Right(&[1, 2, 3][..]);
    let mut all = vec![0];
    all.extend(left.iter());
    all.extend(right.iter());
}

