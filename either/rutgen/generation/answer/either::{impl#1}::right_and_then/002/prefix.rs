// Answer 0

#[test]
fn test_right_and_then_with_left_variant() {
    let left: Either<i32, u32> = Left(123);
    let result = left.right_and_then(|x| Right(x * 2));
}

#[test]
fn test_right_and_then_with_left_variant_string() {
    let left: Either<&str, u32> = Left("error");
    let result = left.right_and_then(|x| Right(x * 2));
}

#[test]
fn test_right_and_then_with_left_variant_float() {
    let left: Either<f64, u32> = Left(45.67);
    let result = left.right_and_then(|x| Right(x * 2));
}

#[test]
fn test_right_and_then_with_left_variant_empty() {
    let left: Either<(), u32> = Left(());
    let result = left.right_and_then(|x| Right(x * 2));
}

