// Answer 0

#[test]
fn test_utf8_decode_invalid_single_byte() {
    let test_input: &[u8] = &[0b1111_1111];
    utf8_decode(test_input);
}

#[test]
fn test_utf8_decode_invalid_two_bytes() {
    let test_input: &[u8] = &[0b1111_1111, 0b1000_0000]; // Invalid continuation byte
    utf8_decode(test_input);
}

#[test]
fn test_utf8_decode_invalid_three_bytes() {
    let test_input: &[u8] = &[0b1111_1111, 0b1000_0000, 0b1000_0000]; // Invalid continuation byte
    utf8_decode(test_input);
}

#[test]
fn test_utf8_decode_invalid_four_bytes() {
    let test_input: &[u8] = &[0b1111_1111, 0b1000_0000, 0b1000_0000, 0b1000_0000]; // Invalid continuation byte
    utf8_decode(test_input);
}

