// Answer 0

#[test]
fn test_clear_non_empty() {
    let mut buf = Bytes::from_static(b"hello world");
    buf.clear();
    buf.is_empty();
}

#[test]
fn test_clear_empty() {
    let mut buf = Bytes::new();
    buf.clear();
    buf.is_empty();
}

