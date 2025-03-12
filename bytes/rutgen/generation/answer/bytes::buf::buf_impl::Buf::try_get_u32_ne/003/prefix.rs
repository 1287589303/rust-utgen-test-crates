// Answer 0

#[test]
fn test_try_get_u32_ne_success() {
    let mut buf: &[u8] = b"\xA1\xA0\x09\x08"; // native-endian 0xA10908A1
    let result = buf.try_get_u32_ne();
}

#[test]
fn test_try_get_u32_ne_failure() {
    let mut buf: &[u8] = b"\x08\x09\xA0"; // only 3 bytes available
    let result = buf.try_get_u32_ne();
}

