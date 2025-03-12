// Answer 0

#[test]
fn test_right_with_left() {
    let left: Either<&str, ()> = Left("some value");
    let result = left.right();
}

#[test]
fn test_right_with_left_empty_string() {
    let left: Either<&str, ()> = Left("");
    let result = left.right();
}

#[test]
fn test_right_with_left_large_value() {
    let left: Either<&str, ()> = Left("a very large string value that exceeds typical lengths");
    let result = left.right();
}

