// Answer 0

#[test]
fn test_decode_suffix_invalid_value_error() {
    let input: &[u8] = &[0, 1, 2, 255];
    let input_index: usize = 0;
    let mut output: [u8; 10] = [0; 10];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let decode_allow_trailing_bits: bool = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let _ = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &decode_table,
        decode_allow_trailing_bits,
        padding_mode,
    );
}

