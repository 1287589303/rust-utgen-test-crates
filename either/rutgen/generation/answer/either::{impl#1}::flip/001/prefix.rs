// Answer 0

#[test]
fn test_flip_right_to_left() {
    let right: Either<(), u32> = Right(42);
    let result = right.flip();
}

#[test]
fn test_flip_right_to_left_string() {
    let right: Either<(), &str> = Right("test");
    let result = right.flip();
}

#[test]
fn test_flip_right_to_left_float() {
    let right: Either<(), f64> = Right(3.14);
    let result = right.flip();
}

#[test]
fn test_flip_right_to_left_empty() {
    let right: Either<(), Vec<u8>> = Right(Vec::new());
    let result = right.flip();
}

#[test]
fn test_flip_right_to_left_char() {
    let right: Either<(), char> = Right('a');
    let result = right.flip();
}

