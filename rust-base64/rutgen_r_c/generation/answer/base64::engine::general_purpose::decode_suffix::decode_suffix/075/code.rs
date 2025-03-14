// Answer 0

#[test]
fn test_decode_suffix_with_indifferent_padding_and_no_trailing_bits() {
    let input: &[u8] = &[65, 66, 67, b'=']; // 'ABC=' in Base64
    let input_index = 0;
    let mut output = [0_u8; 3]; // Expected output length for 'ABC'
    let output_index = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0; // 0
        table[b'B' as usize] = 1; // 1
        table[b'C' as usize] = 2; // 2
        // including only valid symbols for simplicity
        table[b'=' as usize] = PAD_BYTE; // PAD_BYTE
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &decode_table,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert_eq!(result, Ok(DecodeMetadata::new(3, None)));
}

#[test]
fn test_decode_suffix_with_canonical_padding_and_invalid_last_symbol() {
    let input: &[u8] = &[65, 66, 67, b'@']; // 'ABC@' in Base64, invalid symbol
    let input_index = 0;
    let mut output = [0_u8; 3];
    let output_index = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0; // 0
        table[b'B' as usize] = 1; // 1
        table[b'C' as usize] = 2; // 2
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &decode_table,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert!(matches!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(3, b'@')))));
}

#[test]
fn test_decode_suffix_with_non_canonical_padding() {
    let input: &[u8] = &[65, 66, b'=', b'=']; // 'AB==' in Base64
    let input_index = 0;
    let mut output = [0_u8; 2]; // Expected output length for 'AB'
    let output_index = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0; // 0
        table[b'B' as usize] = 1; // 1
        // Prepare padding character
        table[b'=' as usize] = PAD_BYTE; // PAD_BYTE
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &decode_table,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert!(matches!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidPadding))));
}

#[test]
fn test_decode_suffix_with_invalid_padding_after_first() {
    let input: &[u8] = &[65, b'=', 66, b'=' ]; // Invalid padding after valid symbol 'A'
    let input_index = 0;
    let mut output = [0_u8; 2]; // Expected output length for 'A'
    let output_index = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0; // 0
        table[b'B' as usize] = 1; // 1
        table[b'=' as usize] = PAD_BYTE; // Prepare padding character
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &decode_table,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert!(matches!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(1, b'=')))));
}

