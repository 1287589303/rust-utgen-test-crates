// Answer 0

#[test]
fn test_decode_suffix_valid_indifferent_no_padding() {
    let input: &[u8] = &[0, 1, 2, 3];
    let input_index: usize = 0;
    let mut output: [u8; 3] = [0; 3];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = [0; 256];
    let decode_allow_trailing_bits: bool = false;
    let padding_mode: DecodePaddingMode = DecodePaddingMode::Indifferent;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

#[test]
fn test_decode_suffix_valid_indifferent_with_padding() {
    let input: &[u8] = &[0, 1, 2, PAD_BYTE];
    let input_index: usize = 0;
    let mut output: [u8; 3] = [0; 3];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = [0; 256];
    let decode_allow_trailing_bits: bool = false;
    let padding_mode: DecodePaddingMode = DecodePaddingMode::Indifferent;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
} 

#[test]
fn test_decode_suffix_valid_indifferent_with_double_padding() {
    let input: &[u8] = &[0, 1, PAD_BYTE, PAD_BYTE];
    let input_index: usize = 0;
    let mut output: [u8; 3] = [0; 3];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = [0; 256];
    let decode_allow_trailing_bits: bool = false;
    let padding_mode: DecodePaddingMode = DecodePaddingMode::Indifferent;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

