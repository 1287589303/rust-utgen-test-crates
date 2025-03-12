// Answer 0

#[test]
fn test_try_get_f64_le_insufficient_bytes_0() {
    let mut buf: &[u8] = &[];
    let result = buf.try_get_f64_le();
}

#[test]
fn test_try_get_f64_le_insufficient_bytes_1() {
    let mut buf: &[u8] = &b"\x33"[..];
    let result = buf.try_get_f64_le();
}

#[test]
fn test_try_get_f64_le_insufficient_bytes_2() {
    let mut buf: &[u8] = &b"\x33\x33"[..];
    let result = buf.try_get_f64_le();
}

#[test]
fn test_try_get_f64_le_insufficient_bytes_3() {
    let mut buf: &[u8] = &b"\x33\x33\x33"[..];
    let result = buf.try_get_f64_le();
}

#[test]
fn test_try_get_f64_le_insufficient_bytes_4() {
    let mut buf: &[u8] = &b"\x33\x33\x33\x33"[..];
    let result = buf.try_get_f64_le();
}

#[test]
fn test_try_get_f64_le_insufficient_bytes_5() {
    let mut buf: &[u8] = &b"\x33\x33\x33\x33\x33"[..];
    let result = buf.try_get_f64_le();
}

#[test]
fn test_try_get_f64_le_insufficient_bytes_6() {
    let mut buf: &[u8] = &b"\x33\x33\x33\x33\x33\x33"[..];
    let result = buf.try_get_f64_le();
}

