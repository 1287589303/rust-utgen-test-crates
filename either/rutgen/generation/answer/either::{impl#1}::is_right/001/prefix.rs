// Answer 0

#[test]
fn test_is_right_with_left_variant() {
    let left_value: Either<i32, &str> = Left(1);
    let result = left_value.is_right();
}

#[test]
fn test_is_right_with_right_variant() {
    let right_value: Either<i32, &str> = Right("the right value");
    let result = right_value.is_right();
}

#[test]
fn test_is_right_with_another_left_variant() {
    let another_left_value: Either<f64, &str> = Left(3.14);
    let result = another_left_value.is_right();
}

#[test]
fn test_is_right_with_another_right_variant() {
    let another_right_value: Either<i32, &str> = Right("another right value");
    let result = another_right_value.is_right();
}

