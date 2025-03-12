// Answer 0

#[test]
fn test_split_ints() {
    struct IntTuple(i32, i32);
    let tuple = IntTuple(1, 1);
    let result = tuple.split();
}

#[test]
fn test_split_zero() {
    struct ZeroTuple(i32, i32);
    let tuple = ZeroTuple(0, 0);
    let result = tuple.split();
}

#[test]
fn test_split_min_max_i32() {
    struct MinMaxTuple(i32, i32);
    let tuple = MinMaxTuple(i32::MIN, i32::MAX);
    let result = tuple.split();
}

#[test]
fn test_split_min_max_f32() {
    struct MinMaxFTuple(f32, f32);
    let tuple = MinMaxFTuple(f32::MIN, f32::MAX);
    let result = tuple.split();
}

#[test]
fn test_split_char_bounds() {
    struct CharTuple(char, char);
    let tuple = CharTuple(char::from(0), char::from(255));
    let result = tuple.split();
}

#[test]
fn test_split_strings() {
    struct StringTuple(String, String);
    let tuple = StringTuple(String::from("first"), String::from("second"));
    let result = tuple.split();
}

