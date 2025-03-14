// Answer 0

#[test]
fn test_decode_suffix_with_full_input() {
    let input: [u8; 4] = [0b00000000, 0b00000000, 0b00000000, 0b00000000]; // All zero input
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let mut output_index: usize = 0;
    let decode_table: [u8; 256] = [0; 256]; // Initialization as a placeholder
    let decode_allow_trailing_bits = true;
    let padding_mode = DecodePaddingMode::RequireNone;

    let result = decode_suffix(
        &input,
        input_index,
        &mut output,
        output_index,
        &decode_table,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert!(result.is_err());
    assert_eq!(result.err().unwrap().into_inner(), DecodeSliceError::OutputSliceTooSmall);
}

#[test]
fn test_decode_suffix_with_empty_input() {
    let input: [u8; 4] = [0; 4];
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let mut output_index: usize = 0;
    let decode_table: [u8; 256] = [0; 256];
    let decode_allow_trailing_bits = true;
    let padding_mode = DecodePaddingMode::RequireNone;

    let result = decode_suffix(
        &input,
        input_index,
        &mut output,
        output_index,
        &decode_table,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert!(result.is_ok());
}

#[test]
fn test_decode_suffix_invalid_length() {
    let input: [u8; 4] = [0b00000001, 0b00000000, 0b00000000, 0b00000000]; // 1 valid byte
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let mut output_index: usize = 0;
    let decode_table: [u8; 256] = [0; 256];
    let decode_allow_trailing_bits = true;
    let padding_mode = DecodePaddingMode::RequireNone;

    let result = decode_suffix(
        &input,
        input_index,
        &mut output,
        output_index,
        &decode_table,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), DecodeError::InvalidLength(input_index + 1).into());
} 

#[test]
fn test_decode_suffix_invalid_byte() {
    let input: [u8; 4] = [255, 255, 255, 255]; // Invalid bytes
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let mut output_index: usize = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // All values are invalid
    let decode_allow_trailing_bits = true;
    let padding_mode = DecodePaddingMode::RequireNone;

    let result = decode_suffix(
        &input,
        input_index,
        &mut output,
        output_index,
        &decode_table,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert!(result.is_err());
    assert_eq!(result.err().unwrap().kind(), DecodeError::InvalidByte(input_index, 255).kind());
}

