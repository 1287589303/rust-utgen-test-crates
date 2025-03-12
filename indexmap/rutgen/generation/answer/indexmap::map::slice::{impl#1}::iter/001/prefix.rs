// Answer 0

#[test]
fn test_iter_non_empty() {
    struct TestKey;
    struct TestValue;

    let entries = [
        Bucket { hash: 0, key: TestKey, value: TestValue },
        Bucket { hash: 0, key: TestKey, value: TestValue },
    ];
    let slice = Slice { entries };

    let _iter = slice.iter();
}

#[test]
fn test_iter_single_element() {
    struct TestKey;
    struct TestValue;

    let entries = [Bucket { hash: 0, key: TestKey, value: TestValue }];
    let slice = Slice { entries };

    let _iter = slice.iter();
}

#[test]
fn test_iter_multiple_elements() {
    struct TestKey;
    struct TestValue;

    let entries = [
        Bucket { hash: 0, key: TestKey, value: TestValue },
        Bucket { hash: 0, key: TestKey, value: TestValue },
        Bucket { hash: 0, key: TestKey, value: TestValue },
    ];
    let slice = Slice { entries };

    let _iter = slice.iter();
}

#[test]
#[should_panic]
fn test_iter_empty() {
    struct TestKey;
    struct TestValue;

    let entries: [Bucket<TestKey, TestValue>; 0] = [];
    let slice = Slice { entries };

    let _iter = slice.iter();
}

