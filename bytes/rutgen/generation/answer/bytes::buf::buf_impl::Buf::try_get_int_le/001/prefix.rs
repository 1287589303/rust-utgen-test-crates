// Answer 0

#[test]
fn test_try_get_int_le_success_1_byte() {
    let mut buf: &[u8] = &b"\x01 hello"[..];
    let result = buf.try_get_int_le(1);
}

#[test]
fn test_try_get_int_le_success_8_bytes() {
    let mut buf: &[u8] = &b"\x01\x02\x03\x04\x05\x06\x07\x08 hello"[..];
    let result = buf.try_get_int_le(8);
}

#[test]
fn test_try_get_int_le_fail_too_few_bytes() {
    let mut buf: &[u8] = &b"\x01\x02"[..];
    let result = buf.try_get_int_le(3);
}

#[test]
#[should_panic]
fn test_try_get_int_le_panics_too_many_bytes() {
    let mut buf: &[u8] = &b"\x01\x02\x03\x04\x05\x06\x07\x08"[..];
    let _ = buf.try_get_int_le(9);
}

