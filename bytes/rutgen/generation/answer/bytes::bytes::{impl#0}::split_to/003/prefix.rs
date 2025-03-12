// Answer 0

#[test]
fn test_split_to_zero() {
    let mut a = Bytes::from_static(b"hello");
    let b = a.split_to(0);
}

#[test]
fn test_split_to_at_length() {
    let mut a = Bytes::from_static(b"hello");
    let b = a.split_to(a.len());
}

#[test]
fn test_split_to_within_bounds() {
    let mut a = Bytes::from_static(b"hello");
    let b = a.split_to(3);
}

