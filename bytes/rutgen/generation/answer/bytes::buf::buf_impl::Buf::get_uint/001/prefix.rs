// Answer 0

#[test]
fn test_get_uint_1_byte() {
    let mut buf: &[u8] = &b"\x01"[..];
    let result = buf.get_uint(1);
}

#[test]
fn test_get_uint_2_bytes() {
    let mut buf: &[u8] = &b"\x01\x02"[..];
    let result = buf.get_uint(2);
}

#[test]
fn test_get_uint_3_bytes() {
    let mut buf: &[u8] = &b"\x01\x02\x03"[..];
    let result = buf.get_uint(3);
}

#[test]
fn test_get_uint_4_bytes() {
    let mut buf: &[u8] = &b"\x01\x02\x03\x04"[..];
    let result = buf.get_uint(4);
}

#[test]
fn test_get_uint_5_bytes() {
    let mut buf: &[u8] = &b"\x01\x02\x03\x04\x05"[..];
    let result = buf.get_uint(5);
}

#[test]
fn test_get_uint_6_bytes() {
    let mut buf: &[u8] = &b"\x01\x02\x03\x04\x05\x06"[..];
    let result = buf.get_uint(6);
}

#[test]
fn test_get_uint_7_bytes() {
    let mut buf: &[u8] = &b"\x01\x02\x03\x04\x05\x06\x07"[..];
    let result = buf.get_uint(7);
}

#[test]
fn test_get_uint_8_bytes() {
    let mut buf: &[u8] = &b"\x01\x02\x03\x04\x05\x06\x07\x08"[..];
    let result = buf.get_uint(8);
}

#[should_panic]
#[test]
fn test_get_uint_too_few_bytes() {
    let mut buf: &[u8] = &b"\x01"[..];
    let result = buf.get_uint(2);
}

#[should_panic]
#[test]
fn test_get_uint_too_many_bytes() {
    let mut buf: &[u8] = &b"\x01\x02\x03"[..];
    let result = buf.get_uint(9);
}

