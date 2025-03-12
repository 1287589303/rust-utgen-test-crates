// Answer 0

#[test]
fn test_fmt_valid_range() {
    let range = ClassBytesRange { start: 0, end: 255 };
    let _output = format!("{:?}", range);
}

#[test]
fn test_fmt_start_equal_end() {
    let range = ClassBytesRange { start: 100, end: 100 };
    let _output = format!("{:?}", range);
}

#[test]
fn test_fmt_start_zero_end_positive() {
    let range = ClassBytesRange { start: 0, end: 100 };
    let _output = format!("{:?}", range);
}

#[test]
fn test_fmt_start_max_end_max() {
    let range = ClassBytesRange { start: 255, end: 255 };
    let _output = format!("{:?}", range);
}

#[test]
#[should_panic]
fn test_fmt_invalid_range_start_greater_than_end() {
    let _range = ClassBytesRange { start: 200, end: 100 }; // invalid case; structure does not enforce this
}

