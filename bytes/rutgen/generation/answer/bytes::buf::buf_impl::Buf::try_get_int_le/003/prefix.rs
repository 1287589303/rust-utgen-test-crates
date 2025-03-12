// Answer 0

#[test]
fn test_try_get_int_le_success_1_byte() {
    let mut buf = &mut &b"\x01"[..];
    let _ = buf.try_get_int_le(1);
}

#[test]
fn test_try_get_int_le_success_2_bytes() {
    let mut buf = &mut &b"\x01\x02"[..];
    let _ = buf.try_get_int_le(2);
}

#[test]
fn test_try_get_int_le_success_3_bytes() {
    let mut buf = &mut &b"\x01\x02\x03"[..];
    let _ = buf.try_get_int_le(3);
}

#[test]
fn test_try_get_int_le_success_4_bytes() {
    let mut buf = &mut &b"\x01\x02\x03\x04"[..];
    let _ = buf.try_get_int_le(4);
}

#[test]
fn test_try_get_int_le_success_5_bytes() {
    let mut buf = &mut &b"\x01\x02\x03\x04\x05"[..];
    let _ = buf.try_get_int_le(5);
}

#[test]
fn test_try_get_int_le_success_6_bytes() {
    let mut buf = &mut &b"\x01\x02\x03\x04\x05\x06"[..];
    let _ = buf.try_get_int_le(6);
}

#[test]
fn test_try_get_int_le_success_7_bytes() {
    let mut buf = &mut &b"\x01\x02\x03\x04\x05\x06\x07"[..];
    let _ = buf.try_get_int_le(7);
}

#[test]
fn test_try_get_int_le_success_8_bytes() {
    let mut buf = &mut &b"\x01\x02\x03\x04\x05\x06\x07\x08"[..];
    let _ = buf.try_get_int_le(8);
}

#[test]
fn test_try_get_int_le_error_too_few_bytes_1() {
    let mut buf = &mut &b"\x01"[..];
    let _ = buf.try_get_int_le(2);
}

#[test]
fn test_try_get_int_le_error_too_few_bytes_2() {
    let mut buf = &mut &b"\x01\x02"[..];
    let _ = buf.try_get_int_le(3);
}

#[test]
fn test_try_get_int_le_error_too_few_bytes_3() {
    let mut buf = &mut &b"\x01\x02\x03"[..];
    let _ = buf.try_get_int_le(4);
}

#[test]
fn test_try_get_int_le_error_too_few_bytes_4() {
    let mut buf = &mut &b"\x01\x02\x03\x04"[..];
    let _ = buf.try_get_int_le(5);
}

#[test]
fn test_try_get_int_le_error_too_few_bytes_5() {
    let mut buf = &mut &b"\x01\x02\x03\x04\x05"[..];
    let _ = buf.try_get_int_le(6);
}

#[test]
fn test_try_get_int_le_error_too_few_bytes_6() {
    let mut buf = &mut &b"\x01\x02\x03\x04\x05\x06"[..];
    let _ = buf.try_get_int_le(7);
}

#[test]
fn test_try_get_int_le_error_too_few_bytes_7() {
    let mut buf = &mut &b"\x01\x02\x03\x04\x05\x06\x07"[..];
    let _ = buf.try_get_int_le(8);
}

#[test]
#[should_panic]
fn test_try_get_int_le_panic_too_many_bytes() {
    let mut buf = &mut &b"\x01\x02\x03\x04\x05\x06\x07\x08"[..];
    let _ = buf.try_get_int_le(9);
}

