// Answer 0

#[test]
fn test_try_get_i32_ne_success() {
    let mut buf: &[u8] = b"\xA1\xA0\x09\x08";
    let result = buf.try_get_i32_ne();
}

#[test]
fn test_try_get_i32_ne_error() {
    let mut buf = &b"\x08\x09\xA0"[..];
    let result = buf.try_get_i32_ne();
}

