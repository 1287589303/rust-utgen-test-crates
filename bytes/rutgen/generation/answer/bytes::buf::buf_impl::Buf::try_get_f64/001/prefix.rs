// Answer 0

#[test]
fn test_try_get_f64_not_enough_bytes() {
    let mut buf = &b""[..];
    let _ = buf.try_get_f64();
}

#[test]
fn test_try_get_f64_one_byte_available() {
    let mut buf = &b"\x00"[..];
    let _ = buf.try_get_f64();
}

#[test]
fn test_try_get_f64_two_bytes_available() {
    let mut buf = &b"\x00\x00"[..];
    let _ = buf.try_get_f64();
}

#[test]
fn test_try_get_f64_three_bytes_available() {
    let mut buf = &b"\x00\x00\x00"[..];
    let _ = buf.try_get_f64();
}

#[test]
fn test_try_get_f64_four_bytes_available() {
    let mut buf = &b"\x00\x00\x00\x00"[..];
    let _ = buf.try_get_f64();
}

#[test]
fn test_try_get_f64_five_bytes_available() {
    let mut buf = &b"\x00\x00\x00\x00\x00"[..];
    let _ = buf.try_get_f64();
}

#[test]
fn test_try_get_f64_six_bytes_available() {
    let mut buf = &b"\x00\x00\x00\x00\x00\x00"[..];
    let _ = buf.try_get_f64();
}

#[test]
fn test_try_get_f64_seven_bytes_available() {
    let mut buf = &b"\x00\x00\x00\x00\x00\x00\x00"[..];
    let _ = buf.try_get_f64();
}

#[test]
fn test_try_get_f64_eight_bytes_available() {
    let mut buf = &b"\x3F\xF3\x33\x33\x33\x33\x33\x33"[..];
    let _ = buf.try_get_f64();
}

