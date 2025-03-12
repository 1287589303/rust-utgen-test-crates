// Answer 0

#[test]
fn test_slice_of_start_is_zero() {
    struct TestRangeArg;

    let range = RangeFrom::<u32>::from(0);
    let s = "test string";
    range.slice_of(s);
}

#[test]
fn test_slice_of_start_equal_to_length() {
    struct TestRangeArg;

    let range = RangeFrom::<u32>::from(11);
    let s = "test string";
    range.slice_of(s);
}

#[should_panic]
fn test_slice_of_start_exceeding_length() {
    struct TestRangeArg;

    let range = RangeFrom::<u32>::from(12);
    let s = "test string";
    range.slice_of(s);
}

