// Answer 0

#[test]
fn test_decode_suffix_invalid_byte_due_to_padding() {
    let input = b"ABCD==";
    let input_index = 0;
    let mut output = [0; 4];
    let output_index = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // Initialized with invalid values
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let bad_padding_index = input_index + 1; // It should be 1 based on the conditions
    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert_eq!(
        result,
        Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(bad_padding_index, b'=')))
    );
}

#[test]
fn test_decode_suffix_invalid_byte_padding_case_1() {
    let input = b"AB=CD";
    let input_index = 0;
    let mut output = [0; 4];
    let output_index = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // Initialized with invalid values
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let bad_padding_index = input_index + 2; // The second '=' at index 2
    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert_eq!(
        result,
        Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(bad_padding_index, b'=')))
    );
}

