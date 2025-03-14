// Answer 0

#[test]
fn test_decode_chunk_8_invalid_byte_sixth_morsel() {
    let decode_table: [u8; 256] = [
        INVALID_VALUE; 256
    ];
    // Set valid values for the decode table for first 6 bytes
    decode_table[b'A' as usize] = 0; // Corresponds to value 0
    decode_table[b'B' as usize] = 1; // Corresponds to value 1
    decode_table[b'C' as usize] = 2; // Corresponds to value 2
    decode_table[b'D' as usize] = 3; // Corresponds to value 3
    decode_table[b'E' as usize] = 4; // Corresponds to value 4
    decode_table[b'F' as usize] = 5; // Corresponds to value 5
    // Set an invalid value for the sixth morsel
    decode_table[b'!' as usize] = INVALID_VALUE; // Corresponds to an invalid character

    let input: &[u8] = b"ABCDEFG!";
    let mut output = [0u8; 6];
    let result = decode_chunk_8(input, 0, &decode_table, &mut output);

    assert_eq!(
        result,
        Err(DecodeError::InvalidByte(6, b'G'))
    );
}

