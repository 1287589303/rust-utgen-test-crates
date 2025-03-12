// Answer 0

#[test]
fn test_get_u32_ne_exactly_4_bytes() {
    let mut buf: &[u8] = b"\x01\x02\x03\x04";
    let value = buf.get_u32_ne();
}

#[test]
#[should_panic]
fn test_get_u32_ne_fewer_than_4_bytes() {
    let mut buf: &[u8] = b"\x01\x02\x03";
    let value = buf.get_u32_ne();
}

#[test]
fn test_get_u32_ne_more_than_4_bytes() {
    let mut buf: &[u8] = b"\x05\x06\x07\x08 hello";
    let value = buf.get_u32_ne();
}

#[test]
fn test_get_u32_ne_big_endian() {
    let mut buf: &[u8] = match cfg!(target_endian = "big") {
        true => b"\x08\x09\xA0\xA1",
        false => b"\xA1\xA0\x09\x08",
    };
    let value = buf.get_u32_ne();
}

#[test]
fn test_get_u32_ne_little_endian() {
    let mut buf: &[u8] = match cfg!(target_endian = "little") {
        true => b"\xA1\xA0\x09\x08",
        false => b"\x08\x09\xA0\xA1",
    };
    let value = buf.get_u32_ne();
}

