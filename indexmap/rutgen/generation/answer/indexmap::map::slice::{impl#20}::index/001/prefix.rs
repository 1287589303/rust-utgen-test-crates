// Answer 0

#[test]
fn test_index_with_valid_zero() {
    let bucket = Bucket { hash: 0u64.into(), key: "key1", value: "value1" };
    let slice = Slice { entries: [bucket] };
    let result = &slice[0];
}

#[test]
fn test_index_with_valid_max() {
    let bucket1 = Bucket { hash: 0u64.into(), key: "key1", value: "value1" };
    let bucket2 = Bucket { hash: 1u64.into(), key: "key2", value: "value2" };
    let slice = Slice { entries: [bucket1, bucket2] };
    let result = &slice[1];
}

#[test]
fn test_index_with_valid_middle() {
    let bucket1 = Bucket { hash: 0u64.into(), key: "key1", value: "value1" };
    let bucket2 = Bucket { hash: 1u64.into(), key: "key2", value: "value2" };
    let slice = Slice { entries: [bucket1, bucket2] };
    let result = &slice[0];
}

