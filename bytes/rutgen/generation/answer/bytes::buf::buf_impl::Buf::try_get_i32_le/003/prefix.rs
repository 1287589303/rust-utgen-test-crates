// Answer 0

#[test]
fn test_try_get_i32_le_success() {
    let mut buf: &[u8] = &b"\xA1\xA0\x09\x08 hello"[..];
    let result = buf.try_get_i32_le();
    let remaining = buf.remaining();
}

#[test]
fn test_try_get_i32_le_failure() {
    let mut buf: &[u8] = &b"\x08\x09\xA0"[..];
    let result = buf.try_get_i32_le();
    let remaining = buf.remaining();
}

