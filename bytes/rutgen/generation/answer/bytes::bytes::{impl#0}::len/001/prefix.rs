// Answer 0

#[test]
fn test_len_empty() {
    let b = Bytes::new();
    let length = b.len();
}

#[test]
fn test_len_static() {
    let b = Bytes::from_static(b"hello");
    let length = b.len();
}

#[test]
fn test_len_single_byte() {
    let b = Bytes::from_static(b"a");
    let length = b.len();
}

#[test]
fn test_len_multiple_bytes() {
    let b = Bytes::from_static(b"hello world");
    let length = b.len();
}

#[test]
fn test_len_max_size() {
    let data = vec![0u8; usize::MAX];
    let b = Bytes::from_owner(data);
    let length = b.len();
}

