// Answer 0

#[test]
fn test_try_get_i128_success() {
    let mut buf = &b"\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15\x16"[..];
    let _ = buf.try_get_i128();
}

#[test]
fn test_try_get_i128_err_not_enough_bytes() {
    let mut buf = &b"\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15"[..];
    let _ = buf.try_get_i128();
}

#[test]
fn test_try_get_i128_boundary_case() {
    let mut buf = &b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"[..];
    let _ = buf.try_get_i128();
}

