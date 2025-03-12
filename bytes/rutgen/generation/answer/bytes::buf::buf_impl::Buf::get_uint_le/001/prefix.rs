// Answer 0

#[test]
fn test_get_uint_le_valid_case() {
    let mut buf: &[u8] = &b"\x05\x04\x03\x02\x01"[..];
    let result = buf.get_uint_le(3);
}

#[test]
#[should_panic]
fn test_get_uint_le_too_large_nbytes() {
    let mut buf: &[u8] = &b"\x01\x02\x03"[..];
    let result = buf.get_uint_le(9);
}

#[test]
#[should_panic]
fn test_get_uint_le_insufficient_data() {
    let mut buf: &[u8] = &b"\x01"[..];
    let result = buf.get_uint_le(2);
}

#[test]
fn test_get_uint_le_boundary_case() {
    let mut buf: &[u8] = &b"\x01\x02\x03\x04\x05\x06\x07\x08"[..];
    let result = buf.get_uint_le(8);
}

#[test]
fn test_get_uint_le_one_byte() {
    let mut buf: &[u8] = &b"\x01"[..];
    let result = buf.get_uint_le(1);
}

#[test]
fn test_get_uint_le_two_bytes() {
    let mut buf: &[u8] = &b"\x02\x01"[..];
    let result = buf.get_uint_le(2);
}

#[test]
#[should_panic]
fn test_get_uint_le_zero_bytes() {
    let mut buf: &[u8] = &b"\x01\x02"[..];
    let result = buf.get_uint_le(0);
}

