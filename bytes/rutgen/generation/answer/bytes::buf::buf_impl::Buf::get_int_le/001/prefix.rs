// Answer 0

#[test]
fn test_get_int_le_1_byte() {
    let mut buf: &[u8] = &b"\x03"[..];
    let result = buf.get_int_le(1);
}

#[test]
fn test_get_int_le_2_bytes() {
    let mut buf: &[u8] = &b"\x02\x01"[..];
    let result = buf.get_int_le(2);
}

#[test]
fn test_get_int_le_3_bytes() {
    let mut buf: &[u8] = &b"\x01\x02\x03"[..];
    let result = buf.get_int_le(3);
}

#[test]
fn test_get_int_le_4_bytes() {
    let mut buf: &[u8] = &b"\x04\x03\x02\x01"[..];
    let result = buf.get_int_le(4);
}

#[test]
fn test_get_int_le_5_bytes() {
    let mut buf: &[u8] = &b"\x05\x04\x03\x02\x01"[..];
    let result = buf.get_int_le(5);
}

#[test]
fn test_get_int_le_6_bytes() {
    let mut buf: &[u8] = &b"\x06\x05\x04\x03\x02\x01"[..];
    let result = buf.get_int_le(6);
}

#[test]
fn test_get_int_le_7_bytes() {
    let mut buf: &[u8] = &b"\x07\x06\x05\x04\x03\x02\x01"[..];
    let result = buf.get_int_le(7);
}

#[test]
fn test_get_int_le_8_bytes() {
    let mut buf: &[u8] = &b"\x08\x07\x06\x05\x04\x03\x02\x01"[..];
    let result = buf.get_int_le(8);
}

#[should_panic]
fn test_get_int_le_too_few_bytes() {
    let mut buf: &[u8] = &b"\x03"[..];
    let result = buf.get_int_le(4);
}

#[should_panic]
fn test_get_int_le_too_much_bytes() {
    let mut buf: &[u8] = &b"\x01\x02\x03"[..];
    let result = buf.get_int_le(9);
}

