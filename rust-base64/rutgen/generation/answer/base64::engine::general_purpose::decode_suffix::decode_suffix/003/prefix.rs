// Answer 0

#[test]
fn test_decode_suffix_with_two_padding_bytes() {
    let input: &[u8] = b"QUJD=="; // Valid base64 input with 2 padding bytes
    let input_index: usize = 0;
    let mut output = [0_u8; 3]; // Enough capacity to hold decoded result
    let output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0; // A = 0
        table[b'Q' as usize] = 16; // Q = 16
        table[b'J' as usize] = 36; // J = 36
        table[b'C' as usize] = 2; // C = 2
        table[b'=' as usize] = PAD_BYTE; // Padding character
        table
    };
    let decode_allow_trailing_bits: bool = true;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

#[test]
fn test_decode_suffix_without_padding() {
    let input: &[u8] = b"QUJD"; // Valid base64 input with no padding
    let input_index: usize = 0;
    let mut output = [0_u8; 3]; // Enough capacity to hold decoded result
    let output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0; // A = 0
        table[b'Q' as usize] = 16; // Q = 16
        table[b'J' as usize] = 36; // J = 36
        table[b'C' as usize] = 2; // C = 2
        table
    };
    let decode_allow_trailing_bits: bool = true;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

