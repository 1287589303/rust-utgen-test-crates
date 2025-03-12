// Answer 0

#[test]
fn test_split_to_at_len() {
    let mut a = Bytes::from_static(b"hello");
    let b = a.split_to(a.len());
}

#[test]
fn test_split_to_at_zero() {
    let mut a = Bytes::from_static(b"hello");
    let b = a.split_to(0);
}

#[test]
fn test_split_to_at_partial() {
    let mut a = Bytes::from_static(b"hello");
    let b = a.split_to(3);
}

