// Answer 0

#[test]
fn test_try_get_i128_success() {
    let mut buf: &[u8] = &b"\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15\x16 hello"[..];
    let _result = buf.try_get_i128();
}

#[test]
fn test_try_get_i128_fail_just_under_size() {
    let mut buf: &[u8] = &b"\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15"[..];
    let _result = buf.try_get_i128();
}

#[test]
fn test_try_get_i128_fail_empty() {
    let mut buf: &[u8] = &b""[..];
    let _result = buf.try_get_i128();
}

#[test]
fn test_try_get_i128_fail_one_byte() {
    let mut buf: &[u8] = &b"\x01"[..];
    let _result = buf.try_get_i128();
}

#[test]
fn test_try_get_i128_fail_two_bytes() {
    let mut buf: &[u8] = &b"\x01\x02"[..];
    let _result = buf.try_get_i128();
}

#[test]
fn test_try_get_i128_fail_fifteen_bytes() {
    let mut buf: &[u8] = &b"\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15"[..];
    let _result = buf.try_get_i128();
}

