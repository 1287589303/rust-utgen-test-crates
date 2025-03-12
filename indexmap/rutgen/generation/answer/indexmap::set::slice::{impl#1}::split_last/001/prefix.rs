// Answer 0

#[test]
fn test_split_last_with_two_entries() {
    let bucket1 = Bucket { hash: HashValue::default(), key: 1, value: "first" };
    let bucket2 = Bucket { hash: HashValue::default(), key: 2, value: "second" };
    let slice = Box::new(Slice { entries: [bucket1, bucket2] });
    assert!(slice.split_last().is_some());
}

#[test]
fn test_split_last_with_three_entries() {
    let bucket1 = Bucket { hash: HashValue::default(), key: 1, value: "first" };
    let bucket2 = Bucket { hash: HashValue::default(), key: 2, value: "second" };
    let bucket3 = Bucket { hash: HashValue::default(), key: 3, value: "third" };
    let slice = Box::new(Slice { entries: [bucket1, bucket2, bucket3] });
    assert!(slice.split_last().is_some());
}

#[test]
fn test_split_last_with_boundary_condition_one_entry() {
    let bucket1 = Bucket { hash: HashValue::default(), key: 1, value: "only" };
    let slice = Box::new(Slice { entries: [bucket1] });
    assert!(slice.split_last().is_none());
}

