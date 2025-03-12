// Answer 0

#[test]
fn test_is_empty_with_new() {
    let b = Bytes::new();
    b.is_empty();
}

#[test]
fn test_is_empty_with_non_empty() {
    let data = [1, 2, 3];
    let b = Bytes::from_static(&data);
    b.is_empty();
}

#[test]
fn test_is_empty_with_max_length() {
    let data = vec![0u8; usize::MAX];
    let b = Bytes::from_owner(data);
    b.is_empty();
}

