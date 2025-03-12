// Answer 0

#[test]
fn test_next_with_empty_left_and_non_empty_right() {
    let right_iter = vec![1, 2, 3].into_iter();
    let either: Either<iter::Empty<()>, _> = Either::Right(right_iter);
    let _ = either.next();
}

#[test]
fn test_next_with_non_empty_left_and_empty_right() {
    let left_iter = vec![1, 2, 3].into_iter();
    let either: Either<_, iter::Empty<()>> = Either::Left(left_iter);
    let _ = either.next();
}

#[test]
fn test_next_with_same_length_left_and_right() {
    let left_iter = vec![1, 2, 3].into_iter();
    let right_iter = vec![4, 5, 6].into_iter();
    let either: Either<_, _> = Either::Right(right_iter);
    let _ = either.next();
    let _ = Either::Left(left_iter).next();
}

#[test]
fn test_next_with_different_length_left_and_right() {
    let left_iter = vec![1, 2].into_iter();
    let right_iter = vec![3, 4, 5].into_iter();
    let either: Either<_, _> = Either::Right(right_iter);
    let _ = either.next();
    let _ = Either::Left(left_iter).next();
}

#[test]
fn test_next_with_empty_both() {
    let left_iter = iter::empty();
    let right_iter = iter::empty();
    let either: Either<_, _> = Either::Right(right_iter);
    let _ = either.next();
}

#[test]
fn test_next_with_large_different_length() {
    let left_iter = (1..=1000).into_iter();
    let right_iter = (1001..=2000).into_iter();
    let either: Either<_, _> = Either::Right(right_iter);
    let _ = either.next();
    let _ = Either::Left(left_iter).next();
}

