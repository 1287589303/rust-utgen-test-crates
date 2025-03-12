// Answer 0

#[test]
fn test_iter_mut_empty() {
    struct TestKey;
    struct TestValue;

    let mut slice = Slice::<TestKey, TestValue>::new_mut();
    let mut iter = slice.iter_mut();
}

#[test]
fn test_iter_mut_single() {
    struct TestKey;
    struct TestValue;

    let mut slice = Slice {
        entries: [Bucket {
            hash: HashValue::default(),
            key: TestKey,
            value: TestValue,
        }],
    };
    let mut iter = slice.iter_mut();
}

#[test]
fn test_iter_mut_multiple() {
    struct TestKey;
    struct TestValue;

    let mut slice = Slice {
        entries: [
            Bucket {
                hash: HashValue::default(),
                key: TestKey,
                value: TestValue,
            },
            Bucket {
                hash: HashValue::default(),
                key: TestKey,
                value: TestValue,
            },
        ],
    };
    let mut iter = slice.iter_mut();
}

#[test]
fn test_iter_mut_different_types() {
    struct TestKey(i32);
    struct TestValue(String);

    let mut slice = Slice {
        entries: [
            Bucket {
                hash: HashValue::default(),
                key: TestKey(1),
                value: TestValue("value1".to_string()),
            },
            Bucket {
                hash: HashValue::default(),
                key: TestKey(2),
                value: TestValue("value2".to_string()),
            },
            Bucket {
                hash: HashValue::default(),
                key: TestKey(3),
                value: TestValue("value3".to_string()),
            },
        ],
    };
    let mut iter = slice.iter_mut();
}

