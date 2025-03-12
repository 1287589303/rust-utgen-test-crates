// Answer 0

#[test]
fn test_from_encoded_range_two_bytes() {
    let start: &[u8] = &[0x80, 0xC2];
    let end: &[u8] = &[0xBF, 0xDF];
    let result = Utf8Sequence::from_encoded_range(start, end);
}

#[test]
fn test_from_encoded_range_two_bytes_boundary() {
    let start: &[u8] = &[0x00, 0x7F];
    let end: &[u8] = &[0x00, 0x7F];
    let result = Utf8Sequence::from_encoded_range(start, end);
}

#[test]
fn test_from_encoded_range_two_bytes_reverse() {
    let start: &[u8] = &[0xC2, 0x80];
    let end: &[u8] = &[0xDF, 0xBF];
    let result = Utf8Sequence::from_encoded_range(start, end);
}

