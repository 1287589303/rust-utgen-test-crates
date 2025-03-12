// Answer 0

#[test]
fn test_get_index_first_element() {
    struct TestSlice {
        entries: [Bucket<i32, i32>; 3],
    }
    
    let slice = TestSlice {
        entries: [
            Bucket { hash: HashValue::default(), key: 1, value: 10 },
            Bucket { hash: HashValue::default(), key: 2, value: 20 },
            Bucket { hash: HashValue::default(), key: 3, value: 30 },
        ],
    };
    
    let result = slice.get_index(0);
}

#[test]
fn test_get_index_last_element() {
    struct TestSlice {
        entries: [Bucket<i32, i32>; 3],
    }

    let slice = TestSlice {
        entries: [
            Bucket { hash: HashValue::default(), key: 1, value: 10 },
            Bucket { hash: HashValue::default(), key: 2, value: 20 },
            Bucket { hash: HashValue::default(), key: 3, value: 30 },
        ],
    };
    
    let result = slice.get_index(2);
}

#[test]
fn test_get_index_out_of_bounds_low() {
    struct TestSlice {
        entries: [Bucket<i32, i32>; 3],
    }

    let slice = TestSlice {
        entries: [
            Bucket { hash: HashValue::default(), key: 1, value: 10 },
            Bucket { hash: HashValue::default(), key: 2, value: 20 },
            Bucket { hash: HashValue::default(), key: 3, value: 30 },
        ],
    };
    
    let result = slice.get_index(3);
}

#[test]
fn test_get_index_out_of_bounds_high() {
    struct TestSlice {
        entries: [Bucket<i32, i32>; 3],
    }

    let slice = TestSlice {
        entries: [
            Bucket { hash: HashValue::default(), key: 1, value: 10 },
            Bucket { hash: HashValue::default(), key: 2, value: 20 },
            Bucket { hash: HashValue::default(), key: 3, value: 30 },
        ],
    };
    
    let result = slice.get_index(4);
}

