// Answer 0

#[test]
fn test_split_off_at_start() {
    let mut b1 = bytes::Bytes::from("hello world");
    b1.split_off(0);
}

#[test]
fn test_split_off_at_middle() {
    let mut b1 = bytes::Bytes::from("hello world");
    b1.split_off(5);
}

#[test]
fn test_split_off_at_end() {
    let mut b1 = bytes::Bytes::from("hello world");
    b1.split_off(11);
}

#[test]
fn test_split_off_at_invalid_negative() {
    let mut b1 = bytes::Bytes::from("hello world");
    b1.split_off(-1);
}

#[should_panic]
fn test_split_off_at_invalid_high() {
    let mut b1 = bytes::Bytes::from("hello world");
    b1.split_off(12);
}

