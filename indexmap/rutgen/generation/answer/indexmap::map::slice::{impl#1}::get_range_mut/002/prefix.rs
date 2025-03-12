// Answer 0

#[test]
fn test_get_range_mut_valid_range() {
    struct TestSlice {
        entries: [Bucket<i32, i32>; 5],
    }

    let slice = Box::new(Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: 0, value: 0 },
            Bucket { hash: HashValue::default(), key: 1, value: 1 },
            Bucket { hash: HashValue::default(), key: 2, value: 2 },
            Bucket { hash: HashValue::default(), key: 3, value: 3 },
            Bucket { hash: HashValue::default(), key: 4, value: 4 },
        ],
    });

    let mut slice_mut = Box::into_inner(slice);
    let _result = slice_mut.get_range_mut(0..3);
}

#[test]
fn test_get_range_mut_full_range() {
    struct TestSlice {
        entries: [Bucket<i32, i32>; 3],
    }

    let slice = Box::new(Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: 0, value: 0 },
            Bucket { hash: HashValue::default(), key: 1, value: 1 },
            Bucket { hash: HashValue::default(), key: 2, value: 2 },
        ],
    });

    let mut slice_mut = Box::into_inner(slice);
    let _result = slice_mut.get_range_mut(..3);
}

#[test]
fn test_get_range_mut_single_element_range() {
    struct TestSlice {
        entries: [Bucket<i32, i32>; 4],
    }

    let slice = Box::new(Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: 0, value: 0 },
            Bucket { hash: HashValue::default(), key: 1, value: 1 },
            Bucket { hash: HashValue::default(), key: 2, value: 2 },
            Bucket { hash: HashValue::default(), key: 3, value: 3 },
        ],
    });

    let mut slice_mut = Box::into_inner(slice);
    let _result = slice_mut.get_range_mut(2..3);
}

#[test]
fn test_get_range_mut_exclusive_end() {
    struct TestSlice {
        entries: [Bucket<i32, i32>; 3],
    }

    let slice = Box::new(Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: 0, value: 0 },
            Bucket { hash: HashValue::default(), key: 1, value: 1 },
            Bucket { hash: HashValue::default(), key: 2, value: 2 },
        ],
    });

    let mut slice_mut = Box::into_inner(slice);
    let _result = slice_mut.get_range_mut(1..=2);
}

