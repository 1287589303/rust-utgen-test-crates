// Answer 0

#[test]
fn test_get_i32_le_valid_case() {
    let mut buf: &[u8] = &b"\xA1\xA0\x09\x08 hello"[..];
    let result = buf.get_i32_le();
}

#[test]
fn test_get_i32_le_boundary_case() {
    let mut buf: &[u8] = &b"\x01\x02\x03\x04"[..];
    let result = buf.get_i32_le();
}

#[should_panic]
#[test]
fn test_get_i32_le_panic_case_fewer_bytes() {
    let mut buf: &[u8] = &b"\x01\x02"[..];
    let result = buf.get_i32_le();
}

