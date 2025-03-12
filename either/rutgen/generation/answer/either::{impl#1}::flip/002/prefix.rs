// Answer 0

#[test]
fn test_flip_left_to_right_integer() {
    let left: Either<i32, ()> = Left(123);
    let _ = left.flip();
}

#[test]
fn test_flip_left_to_right_string() {
    let left: Either<String, ()> = Left(String::from("text"));
    let _ = left.flip();
}

#[test]
fn test_flip_right_to_left_integer() {
    let right: Either<(), i32> = Right(456);
    let _ = right.flip();
}

#[test]
fn test_flip_right_to_left_string() {
    let right: Either<(), String> = Right(String::from("another text"));
    let _ = right.flip();
}

