// Answer 0

#[test]
fn eq_test_non_empty_identical_entries() {
    struct TestKey(i32);
    struct TestValue(i32);
    
    let slice = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: TestKey(1), value: TestValue(10) },
            Bucket { hash: HashValue::default(), key: TestKey(2), value: TestValue(20) },
        ],
    };
    
    let other = [
        (TestKey(1), TestValue(10)),
        (TestKey(2), TestValue(20)),
    ];
    
    let _result = slice.eq(&other);
}

#[test]
fn eq_test_non_empty_different_entries() {
    struct TestKey(i32);
    struct TestValue(i32);
    
    let slice = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: TestKey(1), value: TestValue(10) },
            Bucket { hash: HashValue::default(), key: TestKey(2), value: TestValue(20) },
        ],
    };
    
    let other = [
        (TestKey(3), TestValue(30)),
        (TestKey(4), TestValue(40)),
    ];
    
    let _result = slice.eq(&other);
}

#[test]
fn eq_test_single_element_identical() {
    struct TestKey(i32);
    struct TestValue(i32);
    
    let slice = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: TestKey(1), value: TestValue(10) },
        ],
    };
    
    let other = [
        (TestKey(1), TestValue(10)),
    ];
    
    let _result = slice.eq(&other);
}

#[test]
fn eq_test_single_element_different() {
    struct TestKey(i32);
    struct TestValue(i32);
    
    let slice = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: TestKey(1), value: TestValue(10) },
        ],
    };
    
    let other = [
        (TestKey(1), TestValue(20)),
    ];
    
    let _result = slice.eq(&other);
}

#[test]
fn eq_test_empty_slice() {
    struct TestKey(i32);
    struct TestValue(i32);
    
    let slice = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: TestKey(1), value: TestValue(10) },
        ],
    };
    
    let other: [(TestKey, TestValue); 0] = [];
    
    let _result = slice.eq(&other);
}

