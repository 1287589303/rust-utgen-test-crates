// Answer 0

#[test]
fn test_get_i64_le_exact_buffer_length() {
    let mut buf: &[u8] = &b"\x08\x07\x06\x05\x04\x03\x02\x01"[..];
    let _result = buf.get_i64_le();
}

#[test]
fn test_get_i64_le_more_than_eight_bytes() {
    let mut buf: &[u8] = &b"\x08\x07\x06\x05\x04\x03\x02\x01 hello"[..];
    let _result = buf.get_i64_le();
}

#[should_panic]
#[test]
fn test_get_i64_le_less_than_eight_bytes() {
    let mut buf: &[u8] = &b"\x08\x07\x06\x05"[..];
    let _result = buf.get_i64_le();
}

#[test]
fn test_get_i64_le_sign_check() {
    let mut buf: &[u8] = &b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF"[..];
    let _result = buf.get_i64_le();
}

