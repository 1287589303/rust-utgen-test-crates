// Answer 0

#[test]
fn test_decode_suffix_invalid_padding_zero_padding() {
    let input: &[u8] = &[b'A', b'B', b'C', b'=', b'='];
    let input_index: usize = 0;
    let mut output = [0u8; 4];
    let mut output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'=' as usize] = PAD_BYTE; // Assume '=' is treated as PAD_BYTE
        table
    };
    let decode_allow_trailing_bits = true;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    assert_eq!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(input_index + 0, b'='))));
}

#[test]
fn test_decode_suffix_invalid_padding_one_padding() {
    let input: &[u8] = &[b'/', b'/', b'/', b'='];
    let input_index: usize = 0;
    let mut output = [0u8; 4];
    let mut output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'/' as usize] = 63; // Assuming '/' for demonstration
        table[b'=' as usize] = PAD_BYTE; // Assume '=' is treated as PAD_BYTE
        table
    };
    let decode_allow_trailing_bits = true;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    assert_eq!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(input_index + 0, b'='))));
}

