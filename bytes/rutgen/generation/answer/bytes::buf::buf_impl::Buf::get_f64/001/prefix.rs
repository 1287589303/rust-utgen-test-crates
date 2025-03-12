// Answer 0

#[test]
fn test_get_f64_valid() {
    let mut buf: &[u8] = &b"\x3F\xF3\x33\x33\x33\x33\x33\x33 hello"[..];
    let _ = buf.get_f64();
}

#[test]
#[should_panic]
fn test_get_f64_not_enough_data_7_bytes() {
    let mut buf: &[u8] = &b"\x3F\xF3\x33\x33\x33\x33\x33"[..];
    let _ = buf.get_f64();
}

#[test]
#[should_panic]
fn test_get_f64_not_enough_data_6_bytes() {
    let mut buf: &[u8] = &b"\x3F\xF3\x33\x33\x33\x33"[..];
    let _ = buf.get_f64();
}

#[test]
#[should_panic]
fn test_get_f64_not_enough_data_5_bytes() {
    let mut buf: &[u8] = &b"\x3F\xF3\x33\x33\x33"[..];
    let _ = buf.get_f64();
}

#[test]
#[should_panic]
fn test_get_f64_not_enough_data_4_bytes() {
    let mut buf: &[u8] = &b"\x3F\xF3\x33\x33"[..];
    let _ = buf.get_f64();
}

#[test]
#[should_panic]
fn test_get_f64_not_enough_data_3_bytes() {
    let mut buf: &[u8] = &b"\x3F\xF3\x33"[..];
    let _ = buf.get_f64();
}

#[test]
#[should_panic]
fn test_get_f64_not_enough_data_2_bytes() {
    let mut buf: &[u8] = &b"\x3F\xF3"[..];
    let _ = buf.get_f64();
}

#[test]
#[should_panic]
fn test_get_f64_not_enough_data_1_byte() {
    let mut buf: &[u8] = &b"\x3F"[..];
    let _ = buf.get_f64();
}

#[test]
#[should_panic]
fn test_get_f64_not_enough_data_0_bytes() {
    let mut buf: &[u8] = &b""[..];
    let _ = buf.get_f64();
}

