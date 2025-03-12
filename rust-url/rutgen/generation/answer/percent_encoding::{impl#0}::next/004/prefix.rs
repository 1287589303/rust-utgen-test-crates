// Answer 0

#[test]
fn test_percent_encode_next_non_encoding_first_byte() {
    let ascii_set = AsciiSet::EMPTY;
    let bytes: &[u8] = b"hello, world"; // Non-encoding first byte
    let mut encoder = PercentEncode { bytes, ascii_set: &ascii_set };
    let result = encoder.next();
}

#[test]
fn test_percent_encode_next_non_encoding_in_remaining() {
    let ascii_set = AsciiSet::EMPTY;
    let bytes: &[u8] = b"hello, world"; // Both non-encoding first byte and non-encoding bytes in the remaining
    let mut encoder = PercentEncode { bytes, ascii_set: &ascii_set };
    let first_result = encoder.next(); // Should yield "h"
    let second_result = encoder.next(); // Should yield "ello, w"
}

#[test]
fn test_percent_encode_next_all_non_encoding() {
    let ascii_set = AsciiSet::EMPTY;
    let bytes: &[u8] = b"no encoding here"; // All bytes do not require percent encoding
    let mut encoder = PercentEncode { bytes, ascii_set: &ascii_set };
    let first_result = encoder.next(); // Should yield "n"
    let second_result = encoder.next(); // Should yield "o "
    let third_result = encoder.next(); // Should yield "e"
}

