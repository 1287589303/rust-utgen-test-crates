// Answer 0

#[test]
fn test_first_mut_non_empty() {
    struct TestKey;
    struct TestValue;

    let mut slice = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: TestKey, value: TestValue },
        ],
    };
    let _result = slice.first_mut();
}

#[test]
fn test_first_mut_two_entries() {
    struct TestKey;
    struct TestValue;

    let mut slice = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: TestKey, value: TestValue },
            Bucket { hash: HashValue::default(), key: TestKey, value: TestValue },
        ],
    };
    let _result = slice.first_mut();
}

#[test]
fn test_first_mut_multiple_entries() {
    struct TestKey;
    struct TestValue;

    let mut slice = Slice {
        entries: [
            Bucket { hash: HashValue::default(), key: TestKey, value: TestValue },
            Bucket { hash: HashValue::default(), key: TestKey, value: TestValue },
            Bucket { hash: HashValue::default(), key: TestKey, value: TestValue },
        ],
    };
    let _result = slice.first_mut();
}

#[test]
#[should_panic]
fn test_first_mut_empty() {
    struct TestKey;
    struct TestValue;

    let mut slice = Slice {
        entries: [],
    };
    let _result = slice.first_mut();
}

