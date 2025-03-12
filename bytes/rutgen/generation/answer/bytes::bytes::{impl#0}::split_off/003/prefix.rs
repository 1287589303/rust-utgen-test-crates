// Answer 0

#[test]
fn test_split_off_at_zero() {
    let mut bytes = Bytes::from_static(b"hello world");
    let result = bytes.split_off(0);
}

#[test]
#[should_panic]
fn test_split_off_at_len() {
    let mut bytes = Bytes::from_static(b"hello world");
    let result = bytes.split_off(bytes.len());
}

#[test]
fn test_split_off_in_range() {
    let mut bytes = Bytes::from_static(b"hello world");
    let at = 5;
    let result = bytes.split_off(at);
}

#[test]
#[should_panic]
fn test_split_off_out_of_bounds() {
    let mut bytes = Bytes::from_static(b"hello");
    let result = bytes.split_off(10);
}

#[test]
#[should_panic]
fn test_split_off_on_empty() {
    let mut bytes = Bytes::new();
    let result = bytes.split_off(0);
}

