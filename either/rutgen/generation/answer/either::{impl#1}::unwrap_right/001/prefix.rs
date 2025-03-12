// Answer 0

#[test]
fn test_unwrap_right_with_integer() {
    let right: Either<(), i32> = Right(3);
    let result = right.unwrap_right();
}

#[test]
fn test_unwrap_right_with_string() {
    let right: Either<(), String> = Right(String::from("hello"));
    let result = right.unwrap_right();
}

#[test]
fn test_unwrap_right_with_float() {
    let right: Either<(), f64> = Right(3.14);
    let result = right.unwrap_right();
}

#[test]
fn test_unwrap_right_with_boolean() {
    let right: Either<(), bool> = Right(true);
    let result = right.unwrap_right();
}

#[should_panic]
fn test_unwrap_right_should_panic_on_left() {
    let left: Either<i32, ()> = Left(3);
    let _result = left.unwrap_right();
}

