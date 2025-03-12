// Answer 0

#[test]
fn test_eq_identical_empty_slices() {
    let slice1 = Slice { entries: [] };
    let slice2 = Slice { entries: [] };
    slice1.eq(&slice2);
}

#[test]
fn test_eq_identical_single_entry() {
    let slice1 = Slice {
        entries: [Bucket {
            hash: HashValue::from(1),
            key: 1,
            value: "value1",
        }],
    };
    let slice2 = Slice {
        entries: [Bucket {
            hash: HashValue::from(1),
            key: 1,
            value: "value1",
        }],
    };
    slice1.eq(&slice2);
}

#[test]
fn test_eq_different_single_entry() {
    let slice1 = Slice {
        entries: [Bucket {
            hash: HashValue::from(1),
            key: 1,
            value: "value1",
        }],
    };
    let slice2 = Slice {
        entries: [Bucket {
            hash: HashValue::from(2),
            key: 2,
            value: "value2",
        }],
    };
    slice1.eq(&slice2);
}

#[test]
fn test_eq_identical_multiple_entries() {
    let slice1 = Slice {
        entries: [
            Bucket {
                hash: HashValue::from(1),
                key: 1,
                value: "value1",
            },
            Bucket {
                hash: HashValue::from(2),
                key: 2,
                value: "value2",
            },
        ],
    };
    let slice2 = Slice {
        entries: [
            Bucket {
                hash: HashValue::from(1),
                key: 1,
                value: "value1",
            },
            Bucket {
                hash: HashValue::from(2),
                key: 2,
                value: "value2",
            },
        ],
    };
    slice1.eq(&slice2);
}

#[test]
fn test_eq_different_multiple_entries() {
    let slice1 = Slice {
        entries: [
            Bucket {
                hash: HashValue::from(1),
                key: 1,
                value: "value1",
            },
            Bucket {
                hash: HashValue::from(2),
                key: 2,
                value: "value2",
            },
        ],
    };
    let slice2 = Slice {
        entries: [
            Bucket {
                hash: HashValue::from(1),
                key: 1,
                value: "different_value",
            },
            Bucket {
                hash: HashValue::from(2),
                key: 3,
                value: "value3",
            },
        ],
    };
    slice1.eq(&slice2);
}

#[test]
fn test_eq_partial_match() {
    let slice1 = Slice {
        entries: [
            Bucket {
                hash: HashValue::from(1),
                key: 1,
                value: "value1",
            },
            Bucket {
                hash: HashValue::from(2),
                key: 2,
                value: "value2",
            },
        ],
    };
    let slice2 = Slice {
        entries: [
            Bucket {
                hash: HashValue::from(1),
                key: 1,
                value: "value1",
            },
            Bucket {
                hash: HashValue::from(2),
                key: 2,
                value: "value_diff",
            },
        ],
    };
    slice1.eq(&slice2);
}

