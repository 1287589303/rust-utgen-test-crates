// Answer 0

#[test]
fn test_decode_chunk_4_invalid_byte_at_last_morsel() {
    let input: &[u8] = &[0xAA, 0xBB, 0xCC, 0xFF]; // FF is an invalid byte
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0xAA as usize] = 0x20; // valid morsel
        table[0xBB as usize] = 0x21; // valid morsel
        table[0xCC as usize] = 0x22; // valid morsel
        table
    };
    let mut output = [0u8; 3];

    let result = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
    
    assert_eq!(
        result,
        Err(DecodeError::InvalidByte(index_at_start_of_input + 3, input[3]))
    );
}

