// Answer 0

#[test]
fn test_decode_helper_complete_quads_len_ok() {
    let input: &[u8] = b"VGhpcyBpcyBhIHRlc3Q="; // Base64 for "This is a test"
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 20 };
    let mut output = [0u8; 20];
    let decode_table = &[0u8; 256]; // This should actually contain the base64 decode table
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_helper(input, &estimate, &mut output, decode_table, decode_allow_trailing_bits, padding_mode);
    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.decoded_len, 15); // "This is a test" is 15 bytes
    assert_eq!(metadata.padding_offset, Some(16)); // Padding byte's offset
}

#[test]
fn test_decode_helper_unrolled_chunks() {
    let input: &[u8] = b"U29tZSBkYXRhIGZvciB0ZXN0aW5n"; // Base64 for "Some data for testing"
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 24 };
    let mut output = [0u8; 36]; // To hold decoded data
    let decode_table = &[0u8; 256]; // Base64 decode table needs to be defined
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_helper(input, &estimate, &mut output, decode_table, decode_allow_trailing_bits, padding_mode);
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    assert_eq!(metadata.decoded_len, 24); // Output should have decoded length of 24
    assert_eq!(metadata.padding_offset, None); // No padding in this input
}

#[test]
fn test_decode_helper_partial_chunks() {
    let input: &[u8] = b"U28gdG8gbm93"; // Base64 for "So to now"
    let estimate = GeneralPurposeEstimate { rem: 2, conservative_decoded_len: 10 };
    let mut output = [0u8; 15]; // Sufficient space
    let decode_table = &[0u8; 256]; // Base64 decode table needs to be defined
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireNone;

    let result = decode_helper(input, &estimate, &mut output, decode_table, decode_allow_trailing_bits, padding_mode);
    assert!(result.is_ok());

    let metadata = result.unwrap();
    assert_eq!(metadata.decoded_len, 10); // Output should be 10 bytes
    assert!(metadata.padding_offset.is_none()); // No padding allowed
}

#[test]
fn test_decode_helper_full_input_unrolled() {
    let input: &[u8] = b"QUJDRA=="; // Base64 for "ABCD"
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 4 };
    let mut output = [0u8; 6]; // Enough space for decoded data
    let decode_table = &[0u8; 256]; // Base64 decode table needs to be defined
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_helper(input, &estimate, &mut output, decode_table, decode_allow_trailing_bits, padding_mode);
    assert!(result.is_ok());

    let metadata = result.unwrap();
    assert_eq!(metadata.decoded_len, 4); // Decoded "ABCD" will be 4 bytes
    assert_eq!(metadata.padding_offset, Some(4)); // Padding byte's offset
}

#[test]
fn test_decode_helper_no_chunks_left() {
    let input: &[u8] = b"TWVhbGxv"; // Base64 for "Meallo"
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 8 };
    let mut output = [0u8; 12]; // Enough space
    let decode_table = &[0u8; 256]; // Proper decode table needed
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireNone;

    let result = decode_helper(input, &estimate, &mut output, decode_table, decode_allow_trailing_bits, padding_mode);
    assert!(result.is_ok());

    let metadata = result.unwrap();
    assert_eq!(metadata.decoded_len, 6); // Decoded "Meallo" will be 6 bytes
    assert!(metadata.padding_offset.is_none()); // No padding allowed
}

