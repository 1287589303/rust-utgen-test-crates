// Answer 0

#[test]
fn test_try_get_uint_le_success_1_byte() {
    let mut buf = &mut b"\x01"[..];
    let _ = buf.try_get_uint_le(1);
}

#[test]
fn test_try_get_uint_le_success_2_bytes() {
    let mut buf = &mut b"\x02\x01"[..];
    let _ = buf.try_get_uint_le(2);
}

#[test]
fn test_try_get_uint_le_success_3_bytes() {
    let mut buf = &mut b"\x03\x02\x01"[..];
    let _ = buf.try_get_uint_le(3);
}

#[test]
fn test_try_get_uint_le_success_4_bytes() {
    let mut buf = &mut b"\x04\x03\x02\x01"[..];
    let _ = buf.try_get_uint_le(4);
}

#[test]
fn test_try_get_uint_le_success_5_bytes() {
    let mut buf = &mut b"\x05\x04\x03\x02\x01"[..];
    let _ = buf.try_get_uint_le(5);
}

#[test]
fn test_try_get_uint_le_success_6_bytes() {
    let mut buf = &mut b"\x06\x05\x04\x03\x02\x01"[..];
    let _ = buf.try_get_uint_le(6);
}

#[test]
fn test_try_get_uint_le_success_7_bytes() {
    let mut buf = &mut b"\x07\x06\x05\x04\x03\x02\x01"[..];
    let _ = buf.try_get_uint_le(7);
}

#[test]
fn test_try_get_uint_le_success_8_bytes() {
    let mut buf = &mut b"\x08\x07\x06\x05\x04\x03\x02\x01"[..];
    let _ = buf.try_get_uint_le(8);
}

#[test]
fn test_try_get_uint_le_error_1_byte() {
    let mut buf = &mut b"\x01"[..];
    let _ = buf.try_get_uint_le(2);
}

#[test]
fn test_try_get_uint_le_error_2_bytes() {
    let mut buf = &mut b"\x01\x02"[..];
    let _ = buf.try_get_uint_le(3);
}

#[test]
fn test_try_get_uint_le_error_3_bytes() {
    let mut buf = &mut b"\x01\x02\x03"[..];
    let _ = buf.try_get_uint_le(4);
}

#[test]
fn test_try_get_uint_le_error_4_bytes() {
    let mut buf = &mut b"\x01\x02\x03\x04"[..];
    let _ = buf.try_get_uint_le(5);
}

#[test]
fn test_try_get_uint_le_error_5_bytes() {
    let mut buf = &mut b"\x01\x02\x03\x04\x05"[..];
    let _ = buf.try_get_uint_le(6);
}

#[test]
fn test_try_get_uint_le_error_6_bytes() {
    let mut buf = &mut b"\x01\x02\x03\x04\x05\x06"[..];
    let _ = buf.try_get_uint_le(7);
}

#[test]
fn test_try_get_uint_le_error_7_bytes() {
    let mut buf = &mut b"\x01\x02\x03\x04\x05\x06\x07"[..];
    let _ = buf.try_get_uint_le(8);
}

#[test]
#[should_panic]
fn test_try_get_uint_le_panic_over_8_bytes() {
    let mut buf = &mut b"\x01\x02\x03\x04\x05\x06\x07\x08"[..];
    let _ = buf.try_get_uint_le(9);
}

