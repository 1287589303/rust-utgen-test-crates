// Answer 0

#[test]
fn test_get_int_1_byte_positive() {
    let mut buf: &[u8] = &b"\x7F"[..];
    buf.get_int(1);
}

#[test]
fn test_get_int_1_byte_negative() {
    let mut buf: &[u8] = &b"\x80"[..];
    buf.get_int(1);
}

#[test]
fn test_get_int_2_bytes_positive() {
    let mut buf: &[u8] = &b"\x01\x02"[..];
    buf.get_int(2);
}

#[test]
fn test_get_int_2_bytes_negative() {
    let mut buf: &[u8] = &b"\xFF\xFE"[..];
    buf.get_int(2);
}

#[test]
fn test_get_int_3_bytes_positive() {
    let mut buf: &[u8] = &b"\x01\x02\x03"[..];
    buf.get_int(3);
}

#[test]
fn test_get_int_3_bytes_negative() {
    let mut buf: &[u8] = &b"\xFF\xFE\xFD"[..];
    buf.get_int(3);
}

#[test]
fn test_get_int_4_bytes_positive() {
    let mut buf: &[u8] = &b"\x00\x00\x00\x01"[..];
    buf.get_int(4);
}

#[test]
fn test_get_int_4_bytes_negative() {
    let mut buf: &[u8] = &b"\xFF\xFF\xFF\xFE"[..];
    buf.get_int(4);
}

#[test]
fn test_get_int_5_bytes_positive() {
    let mut buf: &[u8] = &b"\x00\x00\x01\x02\x03"[..];
    buf.get_int(5);
}

#[test]
fn test_get_int_5_bytes_negative() {
    let mut buf: &[u8] = &b"\xFF\xFF\xFF\xFF\xFC"[..];
    buf.get_int(5);
}

#[test]
fn test_get_int_6_bytes_positive() {
    let mut buf: &[u8] = &b"\x00\x00\x00\x00\x00\x01"[..];
    buf.get_int(6);
}

#[test]
fn test_get_int_6_bytes_negative() {
    let mut buf: &[u8] = &b"\xFF\xFF\xFF\xFF\xFF\xFC"[..];
    buf.get_int(6);
}

#[test]
fn test_get_int_7_bytes_positive() {
    let mut buf: &[u8] = &b"\x00\x00\x00\x00\x00\x00\x01"[..];
    buf.get_int(7);
}

#[test]
fn test_get_int_7_bytes_negative() {
    let mut buf: &[u8] = &b"\xFF\xFF\xFF\xFF\xFF\xFF\xFC"[..];
    buf.get_int(7);
}

#[test]
#[should_panic]
fn test_get_int_8_bytes_positive() {
    let mut buf: &[u8] = &b"\x00\x00\x00\x00\x00\x00\x00\x01"[..];
    buf.get_int(8);
}

#[test]
#[should_panic]
fn test_get_int_8_bytes_negative() {
    let mut buf: &[u8] = &b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC"[..];
    buf.get_int(8);
}

#[test]
#[should_panic]
fn test_get_int_insufficient_bytes() {
    let mut buf: &[u8] = &b"\x01"[..];
    buf.get_int(2);
}

