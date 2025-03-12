// Answer 0

#[test]
fn test_get_u64_ne_valid_data_exact() {
    let mut buf: &[u8] = b"\x01\x02\x03\x04\x05\x06\x07\x08";
    let _result = buf.get_u64_ne();
}

#[test]
#[should_panic]
fn test_get_u64_ne_invalid_data_less_than_8_bytes() {
    let mut buf: &[u8] = b"\x01\x02\x03\x04\x05\x06\x07";
    let _result = buf.get_u64_ne();
}

#[test]
fn test_get_u64_ne_valid_data_more_than_8_bytes() {
    let mut buf: &[u8] = b"\x01\x02\x03\x04\x05\x06\x07\x08 extra data";
    let _result = buf.get_u64_ne();
}

#[test]
fn test_get_u64_ne_valid_data_big_endian() {
    let mut buf: &[u8] = b"\x08\x07\x06\x05\x04\x03\x02\x01";
    let _result = buf.get_u64_ne();
}

