// Answer 0

#[test]
#[should_panic]
fn test_split_off_zero() {
    let mut bytes = Bytes::copy_from_slice(b"hello");
    let _split = bytes.split_off(0);
}

#[test]
#[should_panic]
fn test_split_off_out_of_bounds() {
    let mut bytes = Bytes::copy_from_slice(b"hello");
    let _split = bytes.split_off(6); // at > self.len()
}

#[test]
fn test_split_off_boundary() {
    let mut bytes = Bytes::copy_from_slice(b"hello");
    let split = bytes.split_off(4);
    assert_eq!(&bytes[..], b"hell");
    assert_eq!(&split[..], b"o");
}

#[test]
#[should_panic]
fn test_split_off_negative() {
    let mut bytes = Bytes::copy_from_slice(b"hello");
    let _split = bytes.split_off(usize::MAX); // at < 0 (invalid in Rust)
}

