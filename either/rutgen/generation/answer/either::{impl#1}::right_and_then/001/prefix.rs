// Answer 0

#[test]
fn test_right_and_then_with_zero() {
    let right: Either<u32, u32> = Right(0);
    right.right_and_then(|x| Right(x + 10));
}

#[test]
fn test_right_and_then_with_boundary_value() {
    let right: Either<u32, u32> = Right(u32::MAX);
    right.right_and_then(|x| Right(x / 2));
}

#[test]
fn test_right_and_then_with_typical_value() {
    let right: Either<u32, u32> = Right(123);
    right.right_and_then(|x| Right(x * 2));
}

#[test]
fn test_right_and_then_with_alternative_left() {
    let right: Either<u32, &str> = Right(50);
    right.right_and_then(|x| Left("Error"));
}

#[test]
fn test_right_and_then_changes_type() {
    let right: Either<u32, &str> = Right(34);
    right.right_and_then(|x| Right(format!("Value: {}", x)));
}

