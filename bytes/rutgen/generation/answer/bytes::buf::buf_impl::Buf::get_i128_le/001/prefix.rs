// Answer 0

#[test]
fn test_get_i128_le_valid() {
    let mut buf = &b"\x16\x15\x14\x13\x12\x11\x10\x09\x08\x07\x06\x05\x04\x03\x02\x01"[..];
    let result = buf.get_i128_le();
}

#[test]
fn test_get_i128_le_boundary_min() {
    let mut buf = &b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x80"[..];
    let result = buf.get_i128_le();
}

#[test]
fn test_get_i128_le_boundary_max() {
    let mut buf = &b"\x7f\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff"[..];
    let result = buf.get_i128_le();
}

#[test]
#[should_panic]
fn test_get_i128_le_underflow() {
    let mut buf = &b"\x00"[..];
    let result = buf.get_i128_le();
}

#[test]
#[should_panic]
fn test_get_i128_le_exact_limit() {
    let mut buf = &b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"[..];
    let result = buf.get_i128_le();
}

