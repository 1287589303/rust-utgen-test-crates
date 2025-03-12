// Answer 0

#[test]
fn test_try_get_uint_valid_case_1_byte() {
    let mut buf = &b"\x01"[..];
    let result = buf.try_get_uint(1);
    let expected = Ok(1_u64);
    let remaining = buf.remaining();
}

#[test]
fn test_try_get_uint_valid_case_2_bytes() {
    let mut buf = &b"\x01\x02"[..];
    let result = buf.try_get_uint(2);
    let expected = Ok(0x0102_u64);
    let remaining = buf.remaining();
}

#[test]
fn test_try_get_uint_valid_case_3_bytes() {
    let mut buf = &b"\x01\x02\x03"[..];
    let result = buf.try_get_uint(3);
    let expected = Ok(0x010203_u64);
    let remaining = buf.remaining();
}

#[test]
fn test_try_get_uint_valid_case_4_bytes() {
    let mut buf = &b"\x01\x02\x03\x04"[..];
    let result = buf.try_get_uint(4);
    let expected = Ok(0x01020304_u64);
    let remaining = buf.remaining();
}

#[test]
fn test_try_get_uint_valid_case_5_bytes() {
    let mut buf = &b"\x01\x02\x03\x04\x05"[..];
    let result = buf.try_get_uint(5);
    let expected = Ok(0x0102030405_u64);
    let remaining = buf.remaining();
}

#[test]
fn test_try_get_uint_valid_case_6_bytes() {
    let mut buf = &b"\x01\x02\x03\x04\x05\x06"[..];
    let result = buf.try_get_uint(6);
    let expected = Ok(0x010203040506_u64);
    let remaining = buf.remaining();
}

#[test]
fn test_try_get_uint_valid_case_7_bytes() {
    let mut buf = &b"\x01\x02\x03\x04\x05\x06\x07"[..];
    let result = buf.try_get_uint(7);
    let expected = Ok(0x01020304050607_u64);
    let remaining = buf.remaining();
}

#[test]
fn test_try_get_uint_valid_case_8_bytes() {
    let mut buf = &b"\x01\x02\x03\x04\x05\x06\x07\x08"[..];
    let result = buf.try_get_uint(8);
    let expected = Ok(0x0102030405060708_u64);
    let remaining = buf.remaining();
}

#[test]
#[should_panic]
fn test_try_get_uint_invalid_too_many_bytes() {
    let mut buf = &b"\x01\x02\x03\x04\x05\x06\x07"[..];
    let result = buf.try_get_uint(9);
}

#[test]
fn test_try_get_uint_invalid_not_enough_bytes() {
    let mut buf = &b"\x01\x02\x03"[..];
    let result = buf.try_get_uint(4);
    let expected = Err(TryGetError { requested: 4, available: 3 });
    let remaining = buf.remaining();
}

#[test]
fn test_try_get_uint_zero_bytes() {
    let mut buf = &b""[..];
    let result = buf.try_get_uint(0);
    let expected = Ok(0_u64);
    let remaining = buf.remaining();
}

