// Answer 0

#[test]
fn test_try_get_i128_ne_success() {
    let mut buf: &[u8] = match cfg!(target_endian = "big") {
        true => b"\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15\x16",
        false => b"\x16\x15\x14\x13\x12\x11\x10\x09\x08\x07\x06\x05\x04\x03\x02\x01",
    };
    let _ = buf.try_get_i128_ne();
}

#[test]
fn test_try_get_i128_ne_insufficient_bytes() {
    let mut buf = &b"\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15"[..];
    let _ = buf.try_get_i128_ne();
}

#[test]
fn test_try_get_i128_ne_exceeding_bytes() {
    let mut buf: &[u8] = match cfg!(target_endian = "big") {
        true => b"\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19",
        false => b"\x17\x18\x19\x16\x15\x14\x13\x12\x11\x10\x09\x08\x07\x06\x05\x04\x03\x02\x01",
    };
    let _ = buf.try_get_i128_ne();
}

