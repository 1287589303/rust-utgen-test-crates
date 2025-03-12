// Answer 0

#[test]
fn test_try_get_uint_le_1_byte() {
    let mut buf = &mut &b"\x01 hello"[..];
    let result = buf.try_get_uint_le(1);
}

#[test]
fn test_try_get_uint_le_2_bytes() {
    let mut buf = &mut &b"\x02\x01 hello"[..];
    let result = buf.try_get_uint_le(2);
}

#[test]
fn test_try_get_uint_le_3_bytes() {
    let mut buf = &mut &b"\x03\x02\x01 hello"[..];
    let result = buf.try_get_uint_le(3);
}

#[test]
fn test_try_get_uint_le_4_bytes() {
    let mut buf = &mut &b"\x04\x03\x02\x01 hello"[..];
    let result = buf.try_get_uint_le(4);
}

#[test]
fn test_try_get_uint_le_5_bytes() {
    let mut buf = &mut &b"\x05\x04\x03\x02\x01 hello"[..];
    let result = buf.try_get_uint_le(5);
}

#[test]
fn test_try_get_uint_le_6_bytes() {
    let mut buf = &mut &b"\x06\x05\x04\x03\x02\x01 hello"[..];
    let result = buf.try_get_uint_le(6);
}

#[test]
fn test_try_get_uint_le_7_bytes() {
    let mut buf = &mut &b"\x07\x06\x05\x04\x03\x02\x01 hello"[..];
    let result = buf.try_get_uint_le(7);
}

#[should_panic]
#[test]
fn test_try_get_uint_le_8_bytes_panics() {
    let mut buf = &mut &b"\x08\x07\x06\x05\x04\x03\x02\x01 hello"[..];
    let result = buf.try_get_uint_le(8);
} 

#[test]
fn test_try_get_uint_le_error_case() {
    let mut buf = &mut &b"\x01\x02\x03"[..];
    let result = buf.try_get_uint_le(4);
}

