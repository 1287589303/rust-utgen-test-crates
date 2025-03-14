// Answer 0

#[test]
fn test_decode_suffix_with_indifferent_padding() {
    let input: &[u8] = &[0b00111100, 0b00110001, b'=', b'='];
    let input_index = 0;
    let mut output = [0; 3];
    let mut output_index = 0;
    let decode_table: [u8; 256] = [
        INVALID_VALUE; 256
    ];
    decode_table[b'0' as usize] = 52; // For example purposes, filling decode table
    decode_table[b'1' as usize] = 53; // Just an arbitrary matching character
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);

    assert_eq!(result, Ok(DecodeMetadata::new(2, None)));
    assert_eq!(&output[0..2], &[0b00110000, 0b00110001]);
}

#[test]
fn test_decode_suffix_with_canonical_padding() {
    let input: &[u8] = &[0b00111100, 0b00110000, PAD_BYTE, PAD_BYTE];
    let input_index = 0;
    let mut output = [0; 3];
    let mut output_index = 0;
    let decode_table: [u8; 256] = [
        INVALID_VALUE; 256
    ];
    decode_table[b'0' as usize] = 52; // Filling decode table
    decode_table[b'1' as usize] = 53; // Filling decode table for proper decoding
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);

    assert_eq!(result, Ok(DecodeMetadata::new(2, Some(2))));
    assert_eq!(&output[0..2], &[0b00110000, 0b00000000]);
}

#[test]
fn test_decode_suffix_with_no_padding() {
    let input: &[u8] = &[0b00111100, 0b00110001];
    let input_index = 0;
    let mut output = [0; 3];
    let mut output_index = 0;
    let decode_table: [u8; 256] = [
        INVALID_VALUE; 256
    ];
    decode_table[b'0' as usize] = 52; // Filling decode table
    decode_table[b'1' as usize] = 53; // Filling decode table for proper decoding
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireNone;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);

    assert_eq!(result, Ok(DecodeMetadata::new(2, None)));
    assert_eq!(&output[0..2], &[0b00110000, 0b00110001]);
}

