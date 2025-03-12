// Answer 0

#[test]
fn test_decode_chunk_4_invalid_byte_at_third_position() {
    let input: &[u8] = &[0x41, 0x42, 0x43, 0xFF];
    let index_at_start_of_input = 0;
    let decode_table: &[u8; 256] = &[
        INVALID_VALUE, INVALID_VALUE, INVALID_VALUE, INVALID_VALUE, INVALID_VALUE, INVALID_VALUE, INVALID_VALUE, INVALID_VALUE, 
        INVALID_VALUE, INVALID_VALUE, INVALID_VALUE, INVALID_VALUE, INVALID_VALUE, INVALID_VALUE, INVALID_VALUE, INVALID_VALUE, 
        INVALID_VALUE, INVALID_VALUE, INVALID_VALUE, INVALID_VALUE, INVALID_VALUE, INVALID_VALUE, INVALID_VALUE, INVALID_VALUE, 
        // Valid decode table entries for 'A', 'B', 'C', and other non-invalid values
        0x00, 0x01, 0x02, // ...up to the map for valid base64
        0x41, 0x42, 0x43, INVALID_VALUE, // Where 'A' is at index 65 which is 0x41
        // Fill with necessary valid values ...
        // Remember, 0xFF should map to INVALID_VALUE
        INVALID_VALUE, // 0xFF = 255
    ];
    let mut output = [0u8; 3];

    let _ = decode_chunk_4(input, index_at_start_of_input, decode_table, &mut output);
}

