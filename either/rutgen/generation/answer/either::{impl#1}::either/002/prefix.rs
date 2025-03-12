// Answer 0

#[test]
fn test_either_left_zero() {
    let left: Either<u32, i32> = Left(0);
    let result = left.either(|n| (n * n) as i32, |n| -n);
}

#[test]
fn test_either_left_one() {
    let left: Either<u32, i32> = Left(1);
    let result = left.either(|n| (n * n) as i32, |n| -n);
}

#[test]
fn test_either_left_max_u32() {
    let left: Either<u32, i32> = Left(u32::MAX);
    let result = left.either(|n| (n * n) as i32, |n| -n);
}

#[test]
fn test_either_left_large_value() {
    let left: Either<u32, i32> = Left(12345);
    let result = left.either(|n| (n * n) as i32, |n| -n);
}

#[test]
fn test_either_left_negative_function() {
    let left: Either<u32, i32> = Left(3);
    let result = left.either(|n| (n * n) as i32, |n| -n);
}

