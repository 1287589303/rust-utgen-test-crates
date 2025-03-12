// Answer 0

#[test]
fn test_unwrap_left_panics_when_right() {
    let right: Either<i32, &str> = Right("error");
    right.unwrap_left();
}

#[test]
fn test_unwrap_left_panics_when_right_empty_string() {
    let right: Either<i32, &str> = Right("");
    right.unwrap_left();
}

#[test]
fn test_unwrap_left_panics_when_right_zero() {
    let right: Either<i32, i32> = Right(0);
    right.unwrap_left();
}

#[test]
fn test_unwrap_left_panics_when_right_negative_value() {
    let right: Either<i32, i32> = Right(-100);
    right.unwrap_left();
}

#[test]
fn test_unwrap_left_panics_when_right_boolean() {
    let right: Either<i32, bool> = Right(true);
    right.unwrap_left();
}

#[test]
fn test_unwrap_left_panics_when_right_character() {
    let right: Either<i32, char> = Right('a');
    right.unwrap_left();
}

