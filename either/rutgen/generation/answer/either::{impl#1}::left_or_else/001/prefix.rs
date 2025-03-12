// Answer 0

#[test]
fn test_left_or_else_with_right_u32() {
    let right: Either<String, u32> = Right(3);
    let result = right.left_or_else(|x| x.to_string());
}

#[test]
fn test_left_or_else_with_right_float() {
    let right: Either<String, f64> = Right(2.5);
    let result = right.left_or_else(|x| x.to_string());
}

#[test]
fn test_left_or_else_with_right_char() {
    let right: Either<String, char> = Right('a');
    let result = right.left_or_else(|x| x.to_string());
}

#[test]
fn test_left_or_else_with_right_option() {
    let right: Either<String, Option<i32>> = Right(Some(10));
    let result = right.left_or_else(|x| x.unwrap_or(0));
}

