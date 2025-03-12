// Answer 0

#[test]
fn test_try_get_u64_le_success() {
    let mut buf: &[u8] = &b"\x08\x07\x06\x05\x04\x03\x02\x01"[..];
    let result = buf.try_get_u64_le();
    let remaining = buf.remaining();
}

#[test]
fn test_try_get_u64_le_failure() {
    let mut buf: &[u8] = &b"\x08\x07\x06\x05\x04\x03\x02"[..];
    let result = buf.try_get_u64_le();
    let remaining = buf.remaining();
}

