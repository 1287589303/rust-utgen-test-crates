// Answer 0

#[test]
fn test_try_get_f64_ne_no_remaining() {
    let mut buf: &[u8] = &[];
    let result = buf.try_get_f64_ne();
}

#[test]
fn test_try_get_f64_ne_one_byte_remaining() {
    let mut buf: &[u8] = &b"\x00"[..];
    let result = buf.try_get_f64_ne();
}

#[test]
fn test_try_get_f64_ne_seven_bytes_remaining() {
    let mut buf: &[u8] = &b"\x3F\xF3\x33\x33\x33\x33\x33"[..];
    let result = buf.try_get_f64_ne();
}

#[test]
fn test_try_get_f64_ne_eight_bytes_remaining() {
    let mut buf: &[u8] = &b"\x3F\xF3\x33\x33\x33\x33\x33\x33"[..];
    let result = buf.try_get_f64_ne();
}

