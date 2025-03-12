// Answer 0

#[test]
fn test_split_to_non_empty() {
    let mut bytes = Bytes::copy_from_slice(b"hello world");
    let result = bytes.split_to(5);
}

#[test]
fn test_split_to_boundary() {
    let mut bytes = Bytes::copy_from_slice(b"hello world");
    let result = bytes.split_to(5);
    let result2 = bytes.split_to(1);
}

