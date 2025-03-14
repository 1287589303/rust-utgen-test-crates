// Answer 0

#[test]
fn test_decode_suffix_invalid_padding_canonical() {
    let input: &[u8] = b"QUJD=="; // Valid base64 representation, last two equal padding bytes
    let input_index = 0;
    let mut output = [0u8; 4];
    let output_index = 2; // Starting at index 2, makes 2 total bytes to decode
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'Q' as usize] = 16;
        table[b'U' as usize] = 20;
        table[b'J' as usize] = 9;
        table[b'D' as usize] = 3;
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

