// Answer 0

#[test]
fn test_get_i32_valid_4_bytes() {
    let mut buf: &[u8] = &b"\x08\x09\xA0\xA1"[..];
    let result = buf.get_i32();
}

#[test]
#[should_panic]
fn test_get_i32_panic_less_than_4_bytes() {
    let mut buf: &[u8] = &b"\x08\x09"[..];
    let result = buf.get_i32();
}

#[test]
fn test_get_i32_more_than_4_bytes() {
    let mut buf: &[u8] = &b"\x08\x09\xA0\xA1 hello"[..];
    let result = buf.get_i32();
}

#[test]
fn test_get_i32_exactly_4_bytes() {
    let mut buf: &mut [u8] = &mut [0x01, 0x02, 0x03, 0x04];
    let result = buf.get_i32();
}

