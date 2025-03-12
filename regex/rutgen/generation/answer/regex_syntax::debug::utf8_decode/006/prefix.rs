// Answer 0

#[test]
fn test_utf8_decode_invalid_start_byte_1() {
    let bytes = [0b1000_0000];
    let result = utf8_decode(&bytes);
}

#[test]
fn test_utf8_decode_invalid_start_byte_2() {
    let bytes = [0b1111_1111];
    let result = utf8_decode(&bytes);
}

#[test]
fn test_utf8_decode_invalid_sequence_1() {
    let bytes = [0b1101_1111, 0b1111_1111];
    let result = utf8_decode(&bytes);
}

#[test]
fn test_utf8_decode_invalid_sequence_2() {
    let bytes = [0b1110_1111, 0b1111_1111, 0b1111_1111];
    let result = utf8_decode(&bytes);
}

