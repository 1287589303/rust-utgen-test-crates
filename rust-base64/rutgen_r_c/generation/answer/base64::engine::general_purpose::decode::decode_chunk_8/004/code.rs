// Answer 0

#[test]
fn test_decode_chunk_8_invalid_byte_at_index_3() {
    let input: &[u8] = &[0b00000000, 0b00000001, 0b00000010, 0b11111111, 0b00000100, 0b00000101, 0b00000110, 0b00000111];
    let index_at_start_of_input = 0;
    let decode_table: &[u8; 256] = &[0; 256];
    // Set the decode_table to map the valid inputs appropriately
    decode_table[0] = 0; // valid
    decode_table[1] = 1; // valid
    decode_table[2] = 2; // valid
    decode_table[3] = 3; // invalid (this must map to INVALID_VALUE)
    decode_table[4] = 4; // valid
    decode_table[5] = 5; // valid
    decode_table[6] = 6; // valid
    decode_table[7] = 7; // valid

    let mut output = [0u8; 6];
    let result = decode_chunk_8(input, index_at_start_of_input, decode_table, &mut output);
    
    assert_eq!(
        result,
        Err(DecodeError::InvalidByte(
            index_at_start_of_input + 3,
            input[3]
        ))
    );
}

