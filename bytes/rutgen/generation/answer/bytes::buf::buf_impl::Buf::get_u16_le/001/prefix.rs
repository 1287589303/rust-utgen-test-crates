// Answer 0

#[test]
fn test_get_u16_le_valid_case() {
    let mut buf: &[u8] = &b"\x09\x08 hello"[..];
    let result = buf.get_u16_le();
}

#[test]
fn test_get_u16_le_boundary_case() {
    let mut buf: &[u8] = &b"\x01\x02"[..];
    let result = buf.get_u16_le();
}

#[test]
#[should_panic]
fn test_get_u16_le_insufficient_data() {
    let mut buf: &[u8] = &b"\x01"[..];
    let result = buf.get_u16_le();
}

#[test]
fn test_get_u16_le_full_byte_range() {
    let mut buf: &[u8] = &b"\xFF\xFF hello"[..];
    let result = buf.get_u16_le();
}

#[test]
fn test_get_u16_le_zero_value() {
    let mut buf: &[u8] = &b"\x00\x00 hello"[..];
    let result = buf.get_u16_le();
}

