// Answer 0

#[test]
fn test_try_get_f32_ne_err() {
    let mut buf = &b"\x3F\x99\x99"[..];
    let _ = buf.try_get_f32_ne();
}

#[test]
fn test_try_get_f32_ne_success() {
    let mut buf = &b"\x9A\x99\x99\x3F"[..];
    let _ = buf.try_get_f32_ne();
}

