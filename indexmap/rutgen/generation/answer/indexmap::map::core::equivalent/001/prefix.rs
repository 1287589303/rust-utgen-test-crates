// Answer 0

#[test]
fn test_equivalent_with_valid_input() {
    struct TestKey;
    impl Equivalent<usize> for TestKey {
        fn equivalent(&self, _: &usize) -> bool {
            true
        }
    }
    
    let entries = vec![Bucket { hash: HashValue::default(), key: 1, value: "value1" }];
    let key = TestKey;
    let func = equivalent(&key, &entries);
    let result = func(&0);
}

#[test]
fn test_equivalent_with_non_matching_key() {
    struct TestKey;
    impl Equivalent<usize> for TestKey {
        fn equivalent(&self, _: &usize) -> bool {
            false
        }
    }
    
    let entries = vec![Bucket { hash: HashValue::default(), key: 2, value: "value2" }];
    let key = TestKey;
    let func = equivalent(&key, &entries);
    let result = func(&0);
}

#[test]
fn test_equivalent_with_multiple_entries() {
    struct TestKey;
    impl Equivalent<usize> for TestKey {
        fn equivalent(&self, _: &usize) -> bool {
            true
        }
    }
    
    let entries = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "first" },
        Bucket { hash: HashValue::default(), key: 2, value: "second" }
    ];
    let key = TestKey;
    let func = equivalent(&key, &entries);
    let result1 = func(&0);
    let result2 = func(&1);
}

#[test]
fn test_equivalent_with_empty_slice() {
    struct TestKey;
    impl Equivalent<usize> for TestKey {
        fn equivalent(&self, _: &usize) -> bool {
            false
        }
    }
    
    let entries: Vec<Bucket<usize, &str>> = vec![];
    let key = TestKey;
    let func = equivalent(&key, &entries);
    // No valid index to test, just call to ensure it compiles
    // let result = func(&0); // Uncommenting this would cause a panic due to empty slice
}

