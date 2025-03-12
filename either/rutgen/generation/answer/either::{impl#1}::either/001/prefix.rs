// Answer 0

#[test]
fn test_either_right_with_positive_integer() {
    let right: Either<u32, i32> = Right(5);
    let result = right.either(|n| n * 2, |n| -n);
}

#[test]
fn test_either_right_with_negative_integer() {
    let right: Either<u32, i32> = Right(-3);
    let result = right.either(|n| n * 2, |n| -n);
}

#[test]
fn test_either_right_with_zero() {
    let right: Either<u32, i32> = Right(0);
    let result = right.either(|n| n * 2, |n| -n);
}

