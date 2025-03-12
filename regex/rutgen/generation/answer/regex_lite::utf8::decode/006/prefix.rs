// Answer 0

#[test]
fn test_decode_multi_byte_utf8_2_bytes() {
    let input = [0xC2, 0xA0]; // U+00A0 (non-breaking space)
    let result = decode(&input);
}

#[test]
fn test_decode_multi_byte_utf8_3_bytes() {
    let input = [0xE2, 0x82, 0xAC]; // U+20AC (euro sign)
    let result = decode(&input);
}

#[test]
fn test_decode_multi_byte_utf8_4_bytes() {
    let input = [0xF0, 0x90, 0x8D, 0x88]; // U+10348 (gothic letter hwair)
    let result = decode(&input);
}

