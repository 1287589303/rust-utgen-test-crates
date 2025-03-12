// Answer 0

#[test]
fn test_binary_search_by_found_first() {
    struct TestKey(i32);
    struct TestValue(i32);

    let entries = vec![
        Bucket { hash: HashValue::default(), key: TestKey(1), value: TestValue(10) },
        Bucket { hash: HashValue::default(), key: TestKey(2), value: TestValue(20) },
        Bucket { hash: HashValue::default(), key: TestKey(3), value: TestValue(30) },
    ];

    let slice = Box::new(Slice { entries: entries });

    let result = slice.binary_search_by(|key, _| if key.0 < 2 { Ordering::Less } else if key.0 > 2 { Ordering::Greater } else { Ordering::Equal });
}

#[test]
fn test_binary_search_by_not_found() {
    struct TestKey(i32);
    struct TestValue(i32);

    let entries = vec![
        Bucket { hash: HashValue::default(), key: TestKey(1), value: TestValue(10) },
        Bucket { hash: HashValue::default(), key: TestKey(3), value: TestValue(30) },
        Bucket { hash: HashValue::default(), key: TestKey(5), value: TestValue(50) },
    ];

    let slice = Box::new(Slice { entries: entries });

    let result = slice.binary_search_by(|key, _| if key.0 < 4 { Ordering::Less } else { Ordering::Greater });
}

#[test]
fn test_binary_search_by_found_last() {
    struct TestKey(i32);
    struct TestValue(i32);

    let entries = vec![
        Bucket { hash: HashValue::default(), key: TestKey(1), value: TestValue(10) },
        Bucket { hash: HashValue::default(), key: TestKey(2), value: TestValue(20) },
        Bucket { hash: HashValue::default(), key: TestKey(3), value: TestValue(30) },
    ];

    let slice = Box::new(Slice { entries: entries });

    let result = slice.binary_search_by(|key, _| if key.0 < 3 { Ordering::Less } else if key.0 > 3 { Ordering::Greater } else { Ordering::Equal });
}

#[test]
fn test_binary_search_by_edge_case_empty() {
    struct TestKey(i32);
    struct TestValue(i32);

    let entries: Vec<Bucket<TestKey, TestValue>> = vec![];

    let slice = Box::new(Slice { entries });

    let result = slice.binary_search_by(|_, _| Ordering::Greater);
}

#[test]
fn test_binary_search_by_single_element_found() {
    struct TestKey(i32);
    struct TestValue(i32);

    let entries = vec![
        Bucket { hash: HashValue::default(), key: TestKey(2), value: TestValue(20) },
    ];

    let slice = Box::new(Slice { entries });

    let result = slice.binary_search_by(|key, _| if key.0 < 2 { Ordering::Less } else if key.0 > 2 { Ordering::Greater } else { Ordering::Equal });
}

#[test]
fn test_binary_search_by_single_element_not_found() {
    struct TestKey(i32);
    struct TestValue(i32);

    let entries = vec![
        Bucket { hash: HashValue::default(), key: TestKey(2), value: TestValue(20) },
    ];

    let slice = Box::new(Slice { entries });

    let result = slice.binary_search_by(|key, _| if key.0 < 1 { Ordering::Less } else { Ordering::Greater });
}

