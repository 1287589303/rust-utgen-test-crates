// Answer 0

#[test]
fn test_decode_suffix_valid_case() {
    let input: &[u8] = b"ABCD";
    let input_index = 0;
    let mut output = [0u8; 4];
    let mut output_index = 0;
    let decode_table: [u8; 256] = [
        // fill with sample decode values, assuming a simplified valid case
        // 'A' -> 0, 'B' -> 1, 'C' -> 2, 'D' -> 3 (base64 decoding)
        0, 1, 2, 3, // ... other values would be filled with appropriate mappings
    ];
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.output_index, 3);
    assert!(metadata.first_padding_offset.is_none());
}

#[test]
fn test_decode_suffix_invalid_padding() {
    let input: &[u8] = b"AB=";
    let input_index = 0;
    let mut output = [0u8; 4];
    let mut output_index = 0;
    let decode_table: [u8; 256] = [
        // fill with sample decode values
        0, 1, 2, 3,  // base64 decode mappings
        // Padding is equal sign '='
        0, // for '='
    ];
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert!(result.is_err());
}

#[test]
fn test_decode_suffix_invalid_length() {
    let input: &[u8] = b"A=";
    let input_index = 0;
    let mut output = [0u8; 4];
    let mut output_index = 0;
    let decode_table: [u8; 256] = [
        // fill with sample decode values
        0, 1, 2, 3,  // assumed base64 decode mappings
    ];
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert!(result.is_err());
}

#[test]
fn test_decode_suffix_trailing_bits() {
    let input: &[u8] = b"ABCD=="; // should fail the no trailing bits
    let input_index = 0;
    let mut output = [0u8; 4];
    let mut output_index = 0;
    let decode_table: [u8; 256] = [
        0, 1, 2, 3, // assumed base64 decode mappings
    ];
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert!(result.is_err());
}

