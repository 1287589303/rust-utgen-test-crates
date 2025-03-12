// Answer 0

#[test]
fn test_new_empty_bytes() {
    let b = Bytes::new();
}

#[test]
fn test_new_empty_bytes_length() {
    let b = Bytes::new();
    let length = b.len();
}

#[test]
fn test_new_empty_bytes_is_empty() {
    let b = Bytes::new();
    let is_empty = b.is_empty();
}

#[test]
fn test_new_empty_bytes_slice() {
    let b = Bytes::new();
    let slice = b.slice(..);
}

