// Answer 0

#[test]
fn test_get_i64_valid_input() {
    let mut buf = &b"\x01\x02\x03\x04\x05\x06\x07\x08"[..];
    let result = buf.get_i64();
}

#[test]
#[should_panic]
fn test_get_i64_not_enough_data_0_bytes() {
    let mut buf: &[u8] = &b""[..];
    let _ = buf.get_i64();
}

#[test]
#[should_panic]
fn test_get_i64_not_enough_data_1_byte() {
    let mut buf = &b"\x01"[..];
    let _ = buf.get_i64();
}

#[test]
#[should_panic]
fn test_get_i64_not_enough_data_2_bytes() {
    let mut buf = &b"\x01\x02"[..];
    let _ = buf.get_i64();
}

#[test]
#[should_panic]
fn test_get_i64_not_enough_data_3_bytes() {
    let mut buf = &b"\x01\x02\x03"[..];
    let _ = buf.get_i64();
}

#[test]
#[should_panic]
fn test_get_i64_not_enough_data_4_bytes() {
    let mut buf = &b"\x01\x02\x03\x04"[..];
    let _ = buf.get_i64();
}

#[test]
#[should_panic]
fn test_get_i64_not_enough_data_5_bytes() {
    let mut buf = &b"\x01\x02\x03\x04\x05"[..];
    let _ = buf.get_i64();
}

#[test]
#[should_panic]
fn test_get_i64_not_enough_data_6_bytes() {
    let mut buf = &b"\x01\x02\x03\x04\x05\x06"[..];
    let _ = buf.get_i64();
}

#[test]
#[should_panic]
fn test_get_i64_not_enough_data_7_bytes() {
    let mut buf = &b"\x01\x02\x03\x04\x05\x06\x07"[..];
    let _ = buf.get_i64();
}

