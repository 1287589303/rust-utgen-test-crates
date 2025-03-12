// Answer 0

#[test]
fn cmp_equal_slices() {
    struct Key(i32);
    struct Value(i32);
  
    let slice1 = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: Key(1), value: Value(10) },
            Bucket { hash: HashValue::default(), key: Key(2), value: Value(20) },
        ],
    };
  
    let slice2 = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: Key(1), value: Value(10) },
            Bucket { hash: HashValue::default(), key: Key(2), value: Value(20) },
        ],
    };

    let _ = slice1.cmp(&slice2);
}

#[test]
fn cmp_non_empty_and_empty_slice() {
    struct Key(i32);
    struct Value(i32);

    let non_empty_slice = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: Key(1), value: Value(10) },
        ],
    };

    let empty_slice = Slice {
        entries: [],
    };

    let _ = non_empty_slice.cmp(&empty_slice);
}

#[test]
fn cmp_different_slices() {
    struct Key(i32);
    struct Value(i32);

    let slice1 = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: Key(1), value: Value(10) },
            Bucket { hash: HashValue::default(), key: Key(2), value: Value(20) },
        ],
    };

    let slice2 = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: Key(2), value: Value(20) },
            Bucket { hash: HashValue::default(), key: Key(1), value: Value(10) },
        ],
    };

    let _ = slice1.cmp(&slice2);
}

#[test]
fn cmp_empty_slices() {
    struct Key(i32);
    struct Value(i32);

    let empty_slice1 = Slice {
        entries: [],
    };

    let empty_slice2 = Slice {
        entries: [],
    };

    let _ = empty_slice1.cmp(&empty_slice2);
}

