// Answer 0

#[test]
fn test_decode_chunk_8_invalid_byte_at_index_4() {
    let input: &[u8] = b"ABCDZEFG"; // 'Z' is not valid in base64
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // Initialize valid base64 values
        for (i, &byte) in b"AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz0123456789+/".iter().enumerate() {
            table[byte as usize] = i as u8;
        }
        table
    };
    let mut output = [0u8; 6];

    let result = decode_chunk_8(input, index_at_start_of_input, &decode_table, &mut output);
    
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input + 4, b'Z')));
}

