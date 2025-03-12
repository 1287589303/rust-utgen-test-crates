// Answer 0

#[test]
fn test_right_or_else_with_zero() {
    let right: Either<String, u32> = Right(0);
    let result = right.right_or_else(|x| x.parse().unwrap());
}

#[test]
fn test_right_or_else_with_one() {
    let right: Either<String, u32> = Right(1);
    let result = right.right_or_else(|x| x.parse().unwrap());
}

#[test]
fn test_right_or_else_with_two() {
    let right: Either<String, u32> = Right(2);
    let result = right.right_or_else(|x| x.parse().unwrap());
}

#[test]
fn test_right_or_else_with_max() {
    let right: Either<String, u32> = Right(u32::MAX);
    let result = right.right_or_else(|x| x.parse().unwrap());
}

