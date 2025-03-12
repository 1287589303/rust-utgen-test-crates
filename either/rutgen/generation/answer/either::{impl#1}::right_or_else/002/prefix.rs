// Answer 0

#[test]
fn test_right_or_else_with_non_empty_string_left() {
    let left: Either<String, u32> = Left("42".to_string());
    let result = left.right_or_else(|x| x.parse().unwrap());
}

#[test]
fn test_right_or_else_with_another_non_empty_string_left() {
    let left: Either<String, f64> = Left("3.14".to_string());
    let result = left.right_or_else(|x| x.parse().unwrap());
}

#[test]
fn test_right_or_else_with_left_empty_string() {
    let left: Either<String, i32> = Left("".to_string());
    let result = left.right_or_else(|x| {
        if x.is_empty() {
            0
        } else {
            x.parse().unwrap()
        }
    });
}

#[test]
fn test_right_or_else_with_valid_case() {
    let left: Either<String, u64> = Left("100".to_string());
    let result = left.right_or_else(|x| x.parse().unwrap());
}

