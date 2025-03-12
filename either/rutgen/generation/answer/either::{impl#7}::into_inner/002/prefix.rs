// Answer 0

#[test]
fn test_into_inner_left_positive() {
    let left: Either<u32, u32> = Left(123);
    let _ = left.into_inner();
}

#[test]
fn test_into_inner_left_zero() {
    let left: Either<i32, i32> = Left(0);
    let _ = left.into_inner();
}

#[test]
fn test_into_inner_left_negative() {
    let left: Either<i32, i32> = Left(-456);
    let _ = left.into_inner();
}

#[test]
fn test_into_inner_left_max() {
    let left: Either<i32, i32> = Left(i32::MAX);
    let _ = left.into_inner();
}

#[test]
fn test_into_inner_right_positive() {
    let right: Either<u32, u32> = Right(123);
    let _ = right.into_inner();
}

#[test]
fn test_into_inner_right_zero() {
    let right: Either<i32, i32> = Right(0);
    let _ = right.into_inner();
}

#[test]
fn test_into_inner_right_negative() {
    let right: Either<i32, i32> = Right(-456);
    let _ = right.into_inner();
}

#[test]
fn test_into_inner_right_max() {
    let right: Either<i32, i32> = Right(i32::MAX);
    let _ = right.into_inner();
}

