// Answer 0

#[test]
fn test_copied_left_integer() {
    let value: i32 = 42;
    let either = Either::Left(&value);
    let result = either.copied();
}

#[test]
fn test_copied_left_character() {
    let value: char = 'a';
    let either = Either::Left(&value);
    let result = either.copied();
}

#[test]
fn test_copied_left_zero_integer() {
    let value: i32 = 0;
    let either = Either::Left(&value);
    let result = either.copied();
}

#[test]
fn test_copied_left_min_integer() {
    let value: i32 = i32::MIN;
    let either = Either::Left(&value);
    let result = either.copied();
}

#[test]
fn test_copied_left_max_integer() {
    let value: i32 = i32::MAX;
    let either = Either::Left(&value);
    let result = either.copied();
}

#[test]
fn test_copied_left_negative_integer() {
    let value: i32 = -1;
    let either = Either::Left(&value);
    let result = either.copied();
}

