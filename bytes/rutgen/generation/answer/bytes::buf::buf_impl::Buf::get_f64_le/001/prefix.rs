// Answer 0

#[test]
fn test_get_f64_le_valid() {
    let mut buf: &[u8] = &b"\x33\x33\x33\x33\x33\x33\xF3\x3F"[..];
    let _ = buf.get_f64_le();
}

#[test]
#[should_panic]
fn test_get_f64_le_too_short() {
    let mut buf: &[u8] = &b"\x33\x33\x33\x33\x33\x33"[..];
    let _ = buf.get_f64_le();
}

#[test]
#[should_panic]
fn test_get_f64_le_empty() {
    let mut buf: &[u8] = &b""[..];
    let _ = buf.get_f64_le();
}

#[test]
fn test_get_f64_le_longer_buffer() {
    let mut buf: &[u8] = &b"\x33\x33\x33\x33\x33\x33\xF3\x3F hello"[..];
    let _ = buf.get_f64_le();
}

