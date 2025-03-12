// Answer 0

#[test]
fn test_get_u32_with_exactly_4_bytes() {
    let mut buf: &[u8] = &b"\x01\x02\x03\x04"[..];
    let result = buf.get_u32();
}

#[test]
fn test_get_u32_with_more_than_4_bytes() {
    let mut buf: &[u8] = &b"\x01\x02\x03\x04\x05"[..];
    let result = buf.get_u32();
}

#[should_panic]
#[test]
fn test_get_u32_with_less_than_4_bytes() {
    let mut buf: &[u8] = &b"\x01\x02\x03"[..];
    let result = buf.get_u32();
}

