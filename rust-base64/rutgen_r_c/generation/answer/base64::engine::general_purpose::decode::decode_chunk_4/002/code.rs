// Answer 0

#[test]
fn test_decode_chunk_4_invalid_byte_at_second_character() {
    let input: &[u8] = b"AB@D"; // The '@' character is not valid in base64 and will trigger an error.
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // Fill the decode_table with valid base64 characters
        for (i, c) in "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".chars().enumerate() {
            table[c as usize] = i as u8;
        }
        table
    };
    let mut output = [0u8; 3];

    let result = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
    
    assert_eq!(
        result,
        Err(DecodeError::InvalidByte(index_at_start_of_input + 1, input[1]))
    );
}

