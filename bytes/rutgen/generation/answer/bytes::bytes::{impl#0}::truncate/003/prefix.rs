// Answer 0

#[test]
fn test_truncate_valid_range_non_promotable() {
    let mut buf = Bytes::from_static(b"hello world");
    let len = 5;
    buf.truncate(len);
}

#[test]
fn test_truncate_boundary_case_non_promotable() {
    let mut buf = Bytes::from_static(b"hello world");
    let len = 1;
    buf.truncate(len);
}

#[test]
fn test_truncate_to_non_promotable() {
    let mut buf = Bytes::from_static(b"hello world");
    let len = 4;
    buf.truncate(len);
}

