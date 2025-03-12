// Answer 0

#[test]
fn test_get_i32_ne_exactly_4_bytes() {
    let mut buf: &[u8] = if cfg!(target_endian = "big") {
        b"\x01\x02\x03\x04"
    } else {
        b"\x04\x03\x02\x01"
    };
    let result = buf.get_i32_ne();
}

#[test]
fn test_get_i32_ne_more_than_4_bytes() {
    let mut buf: &[u8] = if cfg!(target_endian = "big") {
        b"\x01\x02\x03\x04 hello"
    } else {
        b"\x04\x03\x02\x01 hello"
    };
    let result = buf.get_i32_ne();
}

#[should_panic]
#[test]
fn test_get_i32_ne_less_than_4_bytes() {
    let mut buf: &[u8] = b"\x01\x02\x03";
    let result = buf.get_i32_ne();
}

