// Answer 0

#[test]
fn test_decode_suffix_with_invalid_padding_due_to_early_padding() {
    let input: &[u8] = &[0x00, 0x01, PAD_BYTE, PAD_BYTE]; // Padding starts at index 2
    let input_index = 0;
    let mut output = [0u8; 4];
    let output_index = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0x00] = 0; // A valid base64 character
        table[0x01] = 1; // A valid base64 character
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);

    assert!(result.is_err());
    if let Err(DecodeSliceError::DecodeError(err)) = result {
        if let DecodeError::InvalidByte(offset, byte) = err {
            assert_eq!(offset, 2); // Bad padding index
            assert_eq!(byte, PAD_BYTE);
        } else {
            panic!("Expected a DecodeError::InvalidByte");
        }
    } else {
        panic!("Expected an error");
    }
}

#[test]
fn test_decode_suffix_valid_input_with_no_padding() {
    let input: &[u8] = &[0x00, 0x01, 0x02, 0x03]; // No padding
    let input_index = 0;
    let mut output = [0u8; 4];
    let output_index = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0x00] = 0; // A valid base64 character
        table[0x01] = 1; // A valid base64 character
        table[0x02] = 2; // A valid base64 character
        table[0x03] = 3; // A valid base64 character
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireNone;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert!(result.is_ok());
    if let Ok(metadata) = result {
        assert_eq!(metadata.decoded_len, 4);
        assert_eq!(metadata.padding_offset, None);
    }
}

#[test]
fn test_decode_suffix_invalid_padding_with_non_canonical_mode() {
    let input: &[u8] = &[0x00, 0x01, PAD_BYTE]; // Invalid since must have 4 chars with 0 to 2 padding
    let input_index = 0;
    let mut output = [0u8; 4];
    let output_index = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0x00] = 0; // A valid base64 character
        table[0x01] = 1; // A valid base64 character
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert!(result.is_err());
    if let Err(DecodeSliceError::DecodeError(err)) = result {
        if let DecodeError::InvalidLength(len) = err {
            assert_eq!(len, 2); // Invalid length: not enough characters for decode
        } else {
            panic!("Expected a DecodeError::InvalidLength");
        }
    } else {
        panic!("Expected an error");
    }
}

#[test]
fn test_decode_suffix_with_non_canonical_padding() {
    let input: &[u8] = &[0x00, 0x01, PAD_BYTE]; // Non-canonical padding
    let input_index = 0;
    let mut output = [0u8; 4];
    let output_index = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0x00] = 0; // A valid base64 character
        table[0x01] = 1; // A valid base64 character
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);

    assert!(result.is_err());
    if let Err(DecodeSliceError::DecodeError(err)) = result {
        if let DecodeError::InvalidPadding = err {
            // Expected error
        } else {
            panic!("Expected a DecodeError::InvalidPadding");
        }
    } else {
        panic!("Expected an error");
    }
}

