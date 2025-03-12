// Answer 0

#[test]
fn test_right_or_default_with_non_empty_default() {
    let right: Either<String, u32> = Right(42);
    let result = right.right_or_default();
}

#[test]
fn test_right_or_default_with_empty_default() {
    let right: Either<String, String> = Right("right value".to_string());
    let result = right.right_or_default();
}

