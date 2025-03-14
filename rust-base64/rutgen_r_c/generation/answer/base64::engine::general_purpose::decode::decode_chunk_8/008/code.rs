// Answer 0

#[test]
fn test_decode_chunk_8_invalid_byte_at_last_morsel() {
    let input: [u8; 8] = [b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'Z']; // 'Z' is invalid
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // Assign valid Base64 values
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        table[b'E' as usize] = 4;
        table[b'F' as usize] = 5;
        table[b'G' as usize] = 6;
        // ... fill in the rest of the valid Base64 characters accordingly ...
        table
    };

    let mut output = [0u8; 6];
    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
    
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input + 7, input[7])));
}

#[test]
fn test_decode_chunk_8_multiple_invalid_bytes() {
    let input: [u8; 8] = [b'A', b'B', b'C', b'D', b'E', b'F', b'@', b'Z']; // '@' and 'Z' are invalid
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        table[b'E' as usize] = 4;
        table[b'F' as usize] = 5;
        table[b'G' as usize] = 6;
        // ... fill in the rest of the valid Base64 characters accordingly ...
        table
    };

    let mut output = [0u8; 6];
    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
    
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input + 6, input[6])));
}

