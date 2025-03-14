// Answer 0

#[test]
fn test_decode_chunk_8_invalid_value_after_valid_bytes() {
    let input: [u8; 8] = [b'A', b'B', b'C', b'D', b'E', b'F', b'@', b'G']; // '@' is invalid
    let index_at_start_of_input = 0;

    // Fake decode table where valid values correspond to valid Base64 characters
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0; // 0
        table[b'B' as usize] = 1; // 1
        table[b'C' as usize] = 2; // 2
        table[b'D' as usize] = 3; // 3
        table[b'E' as usize] = 4; // 4
        table[b'F' as usize] = 5; // 5
        // Continue filling for all valid Base64 symbols...
        table
    };

    let mut output: [u8; 6] = [0; 6];

    // Call the function and check the expected error
    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);

    assert_eq!(
        result,
        Err(DecodeError::InvalidByte(index_at_start_of_input + 5, input[5]))
    );
}

