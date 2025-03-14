// Answer 0

#[test]
fn test_decode_chunk_4_invalid_value_at_last_byte() {
    const INVALID_VALUE: u8 = 255; // Assuming this is the value representing an invalid byte
    let input: &[u8] = &[0b00000001, 0b00000010, 0b00000011, 0b11111111]; // Last byte is invalid
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0b00000001 as usize] = 0; // Valid value
        table[0b00000010 as usize] = 1; // Valid value
        table[0b00000011 as usize] = 2; // Valid value
        table
    };
    let mut output = [0u8; 3];

    let result = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);

    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input + 3, input[3])));
}

