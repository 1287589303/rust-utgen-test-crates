// Answer 0

#[test]
fn test_try_get_f64_success() {
    let mut buf = &b"\x3F\xF3\x33\x33\x33\x33\x33\x33"[..];
    let _result = buf.try_get_f64();
}

#[test]
fn test_try_get_f64_boundary_exceed() {
    let mut buf = &b"\x3F\xF3\x33\x33\x33\x33\x33"[..];
    let _result = buf.try_get_f64();
}

#[test]
fn test_try_get_f64_boundary_exact() {
    let mut buf = &b"\x3F\x80\x00\x00\x00\x00\x00\x00"[..];
    let _result = buf.try_get_f64();
}

#[test]
fn test_try_get_f64_failure() {
    let mut buf = &b"\x3F\xF3\x33\x33\x33\x33\x33"[..];
    let _result = buf.try_get_f64();
}

#[test]
fn test_try_get_f64_empty() {
    let mut buf = &b""[..];
    let _result = buf.try_get_f64();
}

#[test]
fn test_try_get_f64_almost_full() {
    let mut buf = &b"\x3F\x80\x00\x00\x00\x00\x00"[..];
    let _result = buf.try_get_f64();
}

