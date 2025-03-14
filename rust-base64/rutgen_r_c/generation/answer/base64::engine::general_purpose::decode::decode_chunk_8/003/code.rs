// Answer 0

#[test]
fn test_decode_chunk_8_invalid_byte() {
    let input: &[u8] = &[0, 1, 255, 3, 4, 5, 6, 7]; // 255 is an invalid byte for base64
    let index_at_start_of_input = 2;
    let decode_table: [u8; 256] = {
        let mut table = [0; 256];
        table[0] = 0; // valid encoding for A
        table[1] = 1; // valid encoding for B
        table[255] = INVALID_VALUE; // making 255 invalid
        table
    };
    let mut output: [u8; 6] = [0; 6];

    let result = decode_chunk_8(input, index_at_start_of_input, &decode_table, &mut output);
    
    assert_eq!(
        result,
        Err(DecodeError::InvalidByte(
            index_at_start_of_input + 2,
            input[2],
        ))
    );
}

