// Answer 0

#[test]
fn test_new_with_non_empty_entries() {
    #[derive(Copy, Debug)]
    struct TestKey;
    
    #[derive(Copy, Debug)]
    struct TestValue;
    
    let entries: &[Bucket<TestKey, TestValue>] = &[
        Bucket { hash: HashValue::from(1), key: TestKey, value: TestValue },
        Bucket { hash: HashValue::from(2), key: TestKey, value: TestValue },
    ];
    
    let iter = Iter::new(entries);
}

#[test]
fn test_new_with_empty_entries() {
    let entries: &[Bucket<i32, i32>] = &[];
    
    let iter = Iter::new(entries);
}

