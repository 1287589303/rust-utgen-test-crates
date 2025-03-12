// Answer 0

#[test]
fn test_get_uint_ne_smallest_bytes() {
    let mut buf: &[u8] = b"\x01";
    let result = buf.get_uint_ne(1);
}

#[test]
fn test_get_uint_ne_multiple_bytes() {
    let mut buf: &[u8] = b"\x01\x02\x03";
    let result = buf.get_uint_ne(3);
}

#[test]
fn test_get_uint_ne_maximum_bytes() {
    let mut buf: &[u8] = b"\x01\x02\x03\x04\x05\x06\x07\x08";
    let result = buf.get_uint_ne(8);
}

#[test]
#[should_panic]
fn test_get_uint_ne_too_many_bytes() {
    let mut buf: &[u8] = b"\x01\x02\x03\x04";
    let result = buf.get_uint_ne(9);
}

#[test]
#[should_panic]
fn test_get_uint_ne_not_enough_data() {
    let mut buf: &[u8] = b"\x01";
    let result = buf.get_uint_ne(2);
}

