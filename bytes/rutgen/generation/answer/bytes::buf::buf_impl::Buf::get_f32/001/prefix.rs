// Answer 0

#[test]
fn test_get_f32_valid() {
    let mut buf: &[u8] = &b"\x3F\x99\x99\x9A"[..];
    let _ = buf.get_f32();
}

#[test]
#[should_panic]
fn test_get_f32_too_short() {
    let mut buf: &[u8] = &b"\x3F\x99\x99"[..];
    let _ = buf.get_f32();
}

#[test]
fn test_get_f32_multiple_calls() {
    let mut buf: &[u8] = &b"\x3F\x99\x99\x9A\x40\x20\x00\x00"[..];
    let _ = buf.get_f32();
    let _ = buf.get_f32();
}

