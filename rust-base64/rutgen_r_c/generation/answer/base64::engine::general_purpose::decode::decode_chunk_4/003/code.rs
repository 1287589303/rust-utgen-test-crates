// Answer 0

#[test]
fn test_decode_chunk_4_invalid_byte() {
    let input: &[u8] = &[0b11000011, 0b11000010, 0b11000001, 0b11000000]; // input with valid initial bytes and invalid third byte
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0b11000011 as usize] = 63; // valid mapping for first byte
        table[0b11000010 as usize] = 62; // valid mapping for second byte
        // third byte is intentionally invalid
        table[0b11000001 as usize] = INVALID_VALUE; // invalid mapping for third byte
        table[0b11000000 as usize] = 61; // valid mapping for fourth byte
        table
    };
    let mut output = [0u8; 3];

    let result = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
    
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input + 2, input[2])));
}

