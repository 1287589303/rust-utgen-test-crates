// Answer 0

#[test]
fn test_split_last_mut_non_empty() {
    struct TestKey;
    struct TestValue;

    let mut entries: [Bucket<TestKey, TestValue>; 3] = [
        Bucket { hash: 0, key: TestKey {}, value: TestValue {} },
        Bucket { hash: 1, key: TestKey {}, value: TestValue {} },
        Bucket { hash: 2, key: TestKey {}, value: TestValue {} },
    ];
    
    let mut slice = Slice { entries };
    
    let result = slice.split_last_mut();
}

#[test]
fn test_split_last_mut_single_element() {
    struct TestKey;
    struct TestValue;

    let mut entries: [Bucket<TestKey, TestValue>; 1] = [
        Bucket { hash: 0, key: TestKey {}, value: TestValue {} },
    ];
    
    let mut slice = Slice { entries };
    
    let result = slice.split_last_mut();
}

#[test]
fn test_split_last_mut_multiple_elements() {
    struct TestKey;
    struct TestValue;

    let mut entries: [Bucket<TestKey, TestValue>; 5] = [
        Bucket { hash: 0, key: TestKey {}, value: TestValue {} },
        Bucket { hash: 1, key: TestKey {}, value: TestValue {} },
        Bucket { hash: 2, key: TestKey {}, value: TestValue {} },
        Bucket { hash: 3, key: TestKey {}, value: TestValue {} },
        Bucket { hash: 4, key: TestKey {}, value: TestValue {} },
    ];
    
    let mut slice = Slice { entries };
    
    let result = slice.split_last_mut();
}

