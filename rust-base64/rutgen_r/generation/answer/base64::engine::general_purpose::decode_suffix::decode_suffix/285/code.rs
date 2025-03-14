// Answer 0

#[test]
fn test_decode_suffix_invalid_last_symbol() {
    const INPUT: &[u8] = &[
        0b00010100, // Base64 character representation
        0b11100000, // Base64 character representation
        0b11111111, // Extra bits that will cause an error
        0b00000000, // Padding byte
    ];
    let input_index = 0;
    let mut output = [0u8; 3];
    let mut output_index = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0b00010100] = 20; // Corresponding value
        table[0b11100000] = 56; // Corresponding value
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(
        INPUT,
        input_index,
        &mut output,
        output_index,
        &decode_table,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert!(result.is_err());
    if let Err(DecodeError::InvalidLastSymbol(index, symbol)) = result {
        assert_eq!(index, input_index + 2); // morsels_in_leftover is 2
        assert_eq!(symbol, 0b11111111); // last_symbol
    } else {
        panic!("Expected InvalidLastSymbol error");
    }
}

