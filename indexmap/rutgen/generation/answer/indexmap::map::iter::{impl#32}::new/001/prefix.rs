// Answer 0

#[test]
fn test_drain_new_with_non_empty_vector() {
    use std::hash::{Hash, Hasher};

    #[derive(Hash, Eq, PartialEq, Debug)]
    struct TestKey(u32);
    #[derive(Debug)]
    struct TestValue(String);

    let mut vec: Vec<Bucket<TestKey, TestValue>> = Vec::new();
    vec.push(Bucket {
        hash: HashValue::new(1),
        key: TestKey(1),
        value: TestValue("value1".to_string()),
    });
    vec.push(Bucket {
        hash: HashValue::new(2),
        key: TestKey(2),
        value: TestValue("value2".to_string()),
    });

    let drain = vec.drain(..);
    let result = Drain::new(drain);
}

#[test]
fn test_drain_new_with_empty_vector() {
    use std::hash::{Hash, Hasher};

    #[derive(Hash, Eq, PartialEq, Debug)]
    struct TestKey(u32);
    #[derive(Debug)]
    struct TestValue(String);

    let vec: Vec<Bucket<TestKey, TestValue>> = Vec::new();

    let drain = vec.drain(..);
    let result = Drain::new(drain);
}

