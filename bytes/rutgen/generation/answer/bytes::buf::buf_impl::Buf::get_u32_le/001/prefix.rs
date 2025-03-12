// Answer 0

#[test]
fn test_get_u32_le_valid() {
    let mut buf: &[u8] = &[0xA1, 0xA0, 0x09, 0x08];
    let _result = buf.get_u32_le();
}

#[test]
#[should_panic]
fn test_get_u32_le_insufficient_data() {
    let mut buf: &[u8] = &[0xA1, 0xA0]; // Only 2 bytes available
    let _result = buf.get_u32_le();
}

#[test]
fn test_get_u32_le_exactly_4_bytes() {
    let mut buf: &[u8] = &[0x01, 0x00, 0x00, 0x00];
    let _result = buf.get_u32_le();
}

#[test]
fn test_get_u32_le_more_than_4_bytes() {
    let mut buf: &[u8] = &[0x01, 0x00, 0x00, 0x00, 0xFF, 0xFF];
    let _result = buf.get_u32_le();
}

#[test]
#[should_panic]
fn test_get_u32_le_empty_buffer() {
    let mut buf: &[u8] = &[];
    let _result = buf.get_u32_le();
}

#[test]
#[should_panic]
fn test_get_u32_le_less_than_4_bytes() {
    let mut buf: &[u8] = &[0x01, 0x02, 0x03]; // Only 3 bytes available
    let _result = buf.get_u32_le();
}

