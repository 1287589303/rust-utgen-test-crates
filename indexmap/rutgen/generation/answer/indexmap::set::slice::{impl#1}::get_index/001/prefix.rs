// Answer 0

#[test]
fn test_get_index_valid_index_zero() {
    let slice = Box::new(Slice { entries: [
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
    ]});
    let result = slice.get_index(0);
}

#[test]
fn test_get_index_valid_index_last() {
    let slice = Box::new(Slice { entries: [
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
        Bucket { hash: HashValue::default(), key: 2, value: "b" },
    ]});
    let result = slice.get_index(1);
}

#[test]
fn test_get_index_invalid_index_out_of_bounds_negative() {
    let slice = Box::new(Slice { entries: [
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
    ]});
    let result = slice.get_index(!0); // Using !0 to simulate a negative index.
}

#[test]
fn test_get_index_invalid_index_out_of_bounds_too_high() {
    let slice = Box::new(Slice { entries: [
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
    ]});
    let result = slice.get_index(1);
}

