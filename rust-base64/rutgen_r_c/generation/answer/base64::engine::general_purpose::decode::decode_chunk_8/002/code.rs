// Answer 0

#[test]
fn test_decode_chunk_8_invalid_byte_at_index_1() {
    let input: [u8; 8] = [0b00000000, 0b10000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000];
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0b00000000 as usize] = 0; // valid byte
        table
    };
    let mut output: [u8; 6] = [0; 6];
    
    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
    
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input + 1, input[1])));
}

