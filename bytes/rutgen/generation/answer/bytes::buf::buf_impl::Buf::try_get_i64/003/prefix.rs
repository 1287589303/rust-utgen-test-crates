// Answer 0

#[test]
fn test_try_get_i64_success_1() {
    let mut buf: &[u8] = &b"\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A"[..];
    let _ = buf.try_get_i64();
}

#[test]
fn test_try_get_i64_success_2() {
    let mut buf: &[u8] = &b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x00\x00"[..];
    let _ = buf.try_get_i64();
}

#[test]
fn test_try_get_i64_success_exact() {
    let mut buf: &[u8] = &b"\x10\x20\x30\x40\x50\x60\x70\x80"[..];
    let _ = buf.try_get_i64();
}

#[test]
fn test_try_get_i64_failure() {
    let mut buf: &[u8] = &b"\x01\x02\x03\x04\x05\x06\x07"[..];
    let _ = buf.try_get_i64();
}

#[test]
fn test_try_get_i64_more_than_eight_bytes() {
    let mut buf: &[u8] = &b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B"[..];
    let _ = buf.try_get_i64();
}

