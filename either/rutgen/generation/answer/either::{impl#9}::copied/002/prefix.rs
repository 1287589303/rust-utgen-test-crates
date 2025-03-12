// Answer 0

#[test]
fn test_copied_left() {
    let mut left_value: i32 = 42;
    let either = Either::Left(&mut left_value);
    let result: Either<i32, i32> = either.copied();
}

#[test]
fn test_copied_right() {
    let mut right_value: i32 = 99;
    let either = Either::Right(&mut right_value);
    let result: Either<i32, i32> = either.copied();
}

#[test]
fn test_copied_left_boundary() {
    let mut left_value: i32 = 0;
    let either = Either::Left(&mut left_value);
    let result: Either<i32, i32> = either.copied();
}

#[test]
fn test_copied_right_boundary() {
    let mut right_value: i32 = 0;
    let either = Either::Right(&mut right_value);
    let result: Either<i32, i32> = either.copied();
}

#[test]
fn test_copied_left_negative() {
    let mut left_value: i32 = -1;
    let either = Either::Left(&mut left_value);
    let result: Either<i32, i32> = either.copied();
}

#[test]
fn test_copied_right_negative() {
    let mut right_value: i32 = -1;
    let either = Either::Right(&mut right_value);
    let result: Either<i32, i32> = either.copied();
}

