// Answer 0

#[test]
fn test_index_valid_first_element() {
    let slice = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: 1, value: "a" },
            Bucket { hash: HashValue::default(), key: 2, value: "b" },
        ],
    };
    let _ = &slice[0];
}

#[test]
fn test_index_valid_last_element() {
    let slice = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: 1, value: "a" },
            Bucket { hash: HashValue::default(), key: 2, value: "b" },
        ],
    };
    let _ = &slice[1];
}

#[test]
#[should_panic]
fn test_index_out_of_bounds_too_low() {
    let slice = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: 1, value: "a" },
        ],
    };
    let _ = &slice[usize::MAX];
}

#[test]
#[should_panic]
fn test_index_out_of_bounds_too_high() {
    let slice = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: 1, value: "a" },
        ],
    };
    let _ = &slice[1];
}

