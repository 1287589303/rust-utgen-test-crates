// Answer 0

#[test]
fn test_encode_different_utf8_lengths() {
    let range = ScalarRange { start: 0x0041, end: 0x1F600 }; // 'A' and '😀'
    let mut start_buffer = [0u8; MAX_UTF8_BYTES];
    let mut end_buffer = [0u8; MAX_UTF8_BYTES];

    let _ = range.encode(&mut start_buffer, &mut end_buffer);
}

#[test]
fn test_encode_first_valid_scalar_range() {
    let range = ScalarRange { start: 0x0000, end: 0x0100 }; // '\0' and 'À'
    let mut start_buffer = [0u8; MAX_UTF8_BYTES];
    let mut end_buffer = [0u8; MAX_UTF8_BYTES];

    let _ = range.encode(&mut start_buffer, &mut end_buffer);
}

#[test]
fn test_encode_last_valid_scalar_range() {
    let range = ScalarRange { start: 0x10FFFF, end: 0x10FFFE }; // Last valid unicode range
    let mut start_buffer = [0u8; MAX_UTF8_BYTES];
    let mut end_buffer = [0u8; MAX_UTF8_BYTES];

    let _ = range.encode(&mut start_buffer, &mut end_buffer);
}

#[test]
fn test_encode_non_ascii_range() {
    let range = ScalarRange { start: 0x80, end: 0x2000 }; // Non-ASCII characters
    let mut start_buffer = [0u8; MAX_UTF8_BYTES];
    let mut end_buffer = [0u8; MAX_UTF8_BYTES];

    let _ = range.encode(&mut start_buffer, &mut end_buffer);
}

#[test]
fn test_encode_special_character_range() {
    let range = ScalarRange { start: 0x20AC, end: 0x263A }; // '€' and '☺'
    let mut start_buffer = [0u8; MAX_UTF8_BYTES];
    let mut end_buffer = [0u8; MAX_UTF8_BYTES];

    let _ = range.encode(&mut start_buffer, &mut end_buffer);
}

