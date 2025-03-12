// Answer 0

#[test]
fn test_split_to_at_zero() {
    let mut bytes = Bytes::from_static(b"hello");
    let result = bytes.split_to(0);
}

#[test]
#[should_panic]
fn test_split_to_at_greater_than_len() {
    let mut bytes = Bytes::from_static(b"hello");
    let result = bytes.split_to(6);
}

#[test]
fn test_split_to_at_one() {
    let mut bytes = Bytes::from_static(b"hello");
    let result = bytes.split_to(1);
}

#[test]
fn test_split_to_at_mid() {
    let mut bytes = Bytes::from_static(b"hello");
    let result = bytes.split_to(3);
}

#[test]
fn test_split_to_at_boundary() {
    let mut bytes = Bytes::from_static(b"hello");
    let result = bytes.split_to(5);
}

