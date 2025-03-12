// Answer 0

#[test]
fn test_get_u64_le_with_valid_8_bytes() {
    let mut buf: &[u8] = &b"\x08\x07\x06\x05\x04\x03\x02\x01"[..];
    let result = buf.get_u64_le();
}

#[test]
fn test_get_u64_le_with_valid_9_bytes() {
    let mut buf: &[u8] = &b"\x08\x07\x06\x05\x04\x03\x02\x01\x00"[..];
    let result = buf.get_u64_le();
}

#[test]
fn test_get_u64_le_with_valid_16_bytes() {
    let mut buf: &[u8] = &b"\x08\x07\x06\x05\x04\x03\x02\x01\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF"[..];
    let result = buf.get_u64_le();
}

#[should_panic]
#[test]
fn test_get_u64_le_with_less_than_8_bytes() {
    let mut buf: &[u8] = &b"\x08\x07\x06"[..];
    let result = buf.get_u64_le();
}

