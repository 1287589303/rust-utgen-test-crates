// Answer 0

#[test]
fn test_try_get_f32_le_success() {
    let mut buf = &mut &b"\x9A\x99\x99\x3F hello"[..];
    let result = buf.try_get_f32_le();
}

#[test]
fn test_try_get_f32_le_not_enough_bytes_1() {
    let mut buf = &mut &b"\x3F\x99\x99"[..];
    let result = buf.try_get_f32_le();
}

#[test]
fn test_try_get_f32_le_not_enough_bytes_2() {
    let mut buf = &mut &b"\x9A\x99\x99"[..];
    let result = buf.try_get_f32_le();
}

#[test]
fn test_try_get_f32_le_not_enough_bytes_3() {
    let mut buf = &mut &b""[..];
    let result = buf.try_get_f32_le();
}

