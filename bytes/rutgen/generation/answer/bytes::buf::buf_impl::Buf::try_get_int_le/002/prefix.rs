// Answer 0

#[test]
fn test_try_get_int_le_valid_1_byte() {
    let mut buf = &mut [0x01][..];
    let result = buf.try_get_int_le(1);
    let remaining = buf.remaining();
}

#[test]
fn test_try_get_int_le_valid_2_bytes() {
    let mut buf = &mut [0x02, 0x01][..];
    let result = buf.try_get_int_le(2);
    let remaining = buf.remaining();
}

#[test]
fn test_try_get_int_le_valid_3_bytes() {
    let mut buf = &mut [0x03, 0x02, 0x01][..];
    let result = buf.try_get_int_le(3);
    let remaining = buf.remaining();
}

#[test]
fn test_try_get_int_le_valid_4_bytes() {
    let mut buf = &mut [0x04, 0x03, 0x02, 0x01][..];
    let result = buf.try_get_int_le(4);
    let remaining = buf.remaining();
}

#[test]
fn test_try_get_int_le_valid_5_bytes() {
    let mut buf = &mut [0x05, 0x04, 0x03, 0x02, 0x01][..];
    let result = buf.try_get_int_le(5);
    let remaining = buf.remaining();
}

#[test]
fn test_try_get_int_le_valid_6_bytes() {
    let mut buf = &mut [0x06, 0x05, 0x04, 0x03, 0x02, 0x01][..];
    let result = buf.try_get_int_le(6);
    let remaining = buf.remaining();
}

#[test]
fn test_try_get_int_le_valid_7_bytes() {
    let mut buf = &mut [0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01][..];
    let result = buf.try_get_int_le(7);
    let remaining = buf.remaining();
}

#[test]
fn test_try_get_int_le_valid_8_bytes() {
    let mut buf = &mut [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01][..];
    let result = buf.try_get_int_le(8);
    let remaining = buf.remaining();
}

#[test]
#[should_panic]
fn test_try_get_int_le_invalid_too_many_bytes() {
    let mut buf = &mut [0x01, 0x02, 0x03, 0x04, 0x05][..];
    let _result = buf.try_get_int_le(9);
}

