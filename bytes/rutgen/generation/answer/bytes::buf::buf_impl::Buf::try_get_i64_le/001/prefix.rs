// Answer 0

#[test]
fn test_try_get_i64_le_success() {
    let mut buf: &[u8] = &b"\x08\x07\x06\x05\x04\x03\x02\x01"[..];
    let result = buf.try_get_i64_le();
}

#[test]
fn test_try_get_i64_le_not_enough_data_0() {
    let mut buf: &[u8] = &b""[..];
    let result = buf.try_get_i64_le();
}

#[test]
fn test_try_get_i64_le_not_enough_data_1() {
    let mut buf: &[u8] = &b"\x08"[..];
    let result = buf.try_get_i64_le();
}

#[test]
fn test_try_get_i64_le_not_enough_data_2() {
    let mut buf: &[u8] = &b"\x08\x07"[..];
    let result = buf.try_get_i64_le();
}

#[test]
fn test_try_get_i64_le_not_enough_data_3() {
    let mut buf: &[u8] = &b"\x08\x07\x06"[..];
    let result = buf.try_get_i64_le();
}

#[test]
fn test_try_get_i64_le_not_enough_data_4() {
    let mut buf: &[u8] = &b"\x08\x07\x06\x05"[..];
    let result = buf.try_get_i64_le();
}

#[test]
fn test_try_get_i64_le_not_enough_data_5() {
    let mut buf: &[u8] = &b"\x08\x07\x06\x05\x04"[..];
    let result = buf.try_get_i64_le();
}

#[test]
fn test_try_get_i64_le_not_enough_data_6() {
    let mut buf: &[u8] = &b"\x08\x07\x06\x05\x04\x03"[..];
    let result = buf.try_get_i64_le();
}

#[test]
fn test_try_get_i64_le_not_enough_data_7() {
    let mut buf: &[u8] = &b"\x08\x07\x06\x05\x04\x03\x02"[..];
    let result = buf.try_get_i64_le();
}

