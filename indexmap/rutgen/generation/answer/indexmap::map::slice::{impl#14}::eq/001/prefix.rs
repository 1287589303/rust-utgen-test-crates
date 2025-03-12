// Answer 0

#[test]
fn test_eq_identical_entries() {
    struct KeyType(i32);
    struct ValueType(i32);
    
    let slice = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: KeyType(1), value: ValueType(10) },
            Bucket { hash: HashValue::default(), key: KeyType(2), value: ValueType(20) },
        ],
    };

    let other: [(KeyType, ValueType); 2] = [
        (KeyType(1), ValueType(10)),
        (KeyType(2), ValueType(20)),
    ];

    let result = slice.eq(&other);
}

#[test]
fn test_eq_different_entries() {
    struct KeyType(i32);
    struct ValueType(i32);
    
    let slice = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: KeyType(1), value: ValueType(10) },
            Bucket { hash: HashValue::default(), key: KeyType(2), value: ValueType(20) },
        ],
    };

    let other: [(KeyType, ValueType); 2] = [
        (KeyType(3), ValueType(30)),
        (KeyType(4), ValueType(40)),
    ];

    let result = slice.eq(&other);
}

#[test]
fn test_eq_partial_match() {
    struct KeyType(i32);
    struct ValueType(i32);
    
    let slice = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: KeyType(1), value: ValueType(10) },
            Bucket { hash: HashValue::default(), key: KeyType(2), value: ValueType(20) },
        ],
    };

    let other: [(KeyType, ValueType); 2] = [
        (KeyType(1), ValueType(10)),
        (KeyType(3), ValueType(30)),
    ];

    let result = slice.eq(&other);
}

#[test]
fn test_eq_empty_slice_and_empty_array() {
    struct KeyType(i32);
    struct ValueType(i32);
    
    let slice: Slice<KeyType, ValueType> = Slice { entries: [] };

    let other: [(KeyType, ValueType); 0] = [];

    let result = slice.eq(&other);
}

