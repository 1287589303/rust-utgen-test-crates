// Answer 0

#[test]
fn test_next_with_non_ascii_first_byte() {
    let bytes: &[u8] = &[0xFF, 0x01, 0x02]; // Non-ASCII byte followed by two valid bytes
    let ascii_set = AsciiSet::EMPTY.add(0xFF); // Assuming we want to encode 0xFF
    let mut encoder = PercentEncode { bytes, ascii_set: &ascii_set };
    let result = encoder.next();
    let expected = percent_encode_byte(0xFF);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_next_with_ascii_byte_included_in_ascii_set() {
    let bytes: &[u8] = &[0x21, 0x22, 0x23]; // ASCII bytes, but 0x21 is included in the ascii_set
    let ascii_set = AsciiSet::EMPTY.add(0x21); // Add 0x21 to the ascii set
    let mut encoder = PercentEncode { bytes, ascii_set: &ascii_set };
    let result = encoder.next();
    let expected = percent_encode_byte(0x21);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_next_with_first_byte_not_included_in_ascii_set() {
    let bytes: &[u8] = &[0x20, 0x21, 0x22]; // First byte is an ASCII that is not in the ascii_set
    let ascii_set = AsciiSet::EMPTY.add(0x21); // Only include 0x21 in the ascii set
    let mut encoder = PercentEncode { bytes, ascii_set: &ascii_set };
    // The first byte (0x20) should not trigger encoding
    let result = encoder.next();
    assert_eq!(result, None); // Since 0x20 is not supposed to be encoded, expect None
}

