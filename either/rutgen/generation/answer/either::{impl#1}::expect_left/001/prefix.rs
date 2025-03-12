// Answer 0

#[test]
#[should_panic(expected = "value was Right: 3")]
fn test_expect_left_panic_with_integer() {
    let right: Either<i32, ()> = Right(3);
    right.expect_left("value was Right");
}

#[test]
#[should_panic(expected = "message: \"panic\"")]
fn test_expect_left_panic_with_short_message() {
    let right: Either<(), ()> = Right(4);
    right.expect_left("panic");
}

#[test]
#[should_panic(expected = "error message: \"this is a panic message\"")]
fn test_expect_left_panic_with_long_message() {
    let right: Either<(), ()> = Right(5);
    right.expect_left("error message: this is a panic message");
}

#[test]
#[should_panic(expected = "unexpected result: 100")]
fn test_expect_left_panic_with_custom_message() {
    let right: Either<(), ()> = Right(100);
    right.expect_left("unexpected result");
}

