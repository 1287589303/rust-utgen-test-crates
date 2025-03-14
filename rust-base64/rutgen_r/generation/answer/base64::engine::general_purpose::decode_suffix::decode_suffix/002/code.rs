// Answer 0

#[test]
fn test_decode_suffix_invalid_byte_padding_with_non_padding_characters() {
    const INPUT: &[u8] = b"ABC=";
    let input_index = 0;
    let mut output: [u8; 4] = [0; 4];
    let mut output_index = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let decode_allow_trailing_bits = true;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    match decode_suffix(INPUT, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode) {
        Err(DecodeError::InvalidByte(_, _)) => {}
        _ => panic!("Expected InvalidByte error"),
    }
}

#[test]
fn test_decode_suffix_invalid_byte_padding_with_extra_padding_character() {
    const INPUT: &[u8] = b"ABCD==";
    let input_index = 0;
    let mut output: [u8; 4] = [0; 4];
    let mut output_index = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let decode_allow_trailing_bits = true;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    match decode_suffix(INPUT, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode) {
        Err(DecodeError::InvalidByte(_, _)) => {}
        _ => panic!("Expected InvalidByte error"),
    }
}

#[test]
fn test_decode_suffix_invalid_length_with_not_enough_morsels() {
    const INPUT: &[u8] = b"AB=";
    let input_index = 0;
    let mut output: [u8; 4] = [0; 4];
    let mut output_index = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let decode_allow_trailing_bits = true;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    match decode_suffix(INPUT, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode) {
        Err(DecodeError::InvalidLength(_)) => {}
        _ => panic!("Expected InvalidLength error"),
    }
}

