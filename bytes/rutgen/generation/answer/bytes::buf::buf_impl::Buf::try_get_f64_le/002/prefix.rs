// Answer 0

#[test]
fn test_try_get_f64_le_success() {
    let mut buf: &[u8] = &b"\x00\x00\x00\x00\x00\x00\x00\x00 hello"[..];
    let result = buf.try_get_f64_le();
}

#[test]
fn test_try_get_f64_le_success_with_boundary() {
    let mut buf: &[u8] = &b"\x7F\xF8\x00\x00\x00\x00\x00\x00"[..];
    let result = buf.try_get_f64_le();
}

#[test]
fn test_try_get_f64_le_success_large_buffer() {
    let mut buf: &[u8] = &b"\x40\x09\x21\xf9\x0c\x00\x00\x00\x00"[..];
    let result = buf.try_get_f64_le();
}

#[test]
#[should_panic]
fn test_try_get_f64_le_error_insufficient_data() {
    let mut buf: &[u8] = &b"\x3F\xF3\x33\x33\x33\x33\x33"[..];
    let result = buf.try_get_f64_le();
}

#[test]
fn test_try_get_f64_le_success_large_buffer_with_extra_bytes() {
    let mut buf: &[u8] = &b"\x40\x09\x21\xf9\x0c\x00\x00\x00\x00\x00\x00"[..];
    let result = buf.try_get_f64_le();
}

