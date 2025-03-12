// Answer 0

#[test]
fn test_right_or_default_with_left() {
    let left: Either<String, u32> = Left("left".to_string());
    let result = left.right_or_default();
}

#[test]
fn test_right_or_default_with_left_empty_string() {
    let left: Either<String, u32> = Left("".to_string());
    let result = left.right_or_default();
}

#[test]
fn test_right_or_default_with_left_zero_length_string() {
    let left: Either<String, u32> = Left("zero length".to_string());
    let result = left.right_or_default();
}

#[test]
fn test_right_or_default_with_left_numeric_string() {
    let left: Either<String, u32> = Left("123".to_string());
    let result = left.right_or_default();
}

