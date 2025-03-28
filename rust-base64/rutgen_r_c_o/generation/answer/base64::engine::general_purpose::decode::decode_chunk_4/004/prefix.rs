// Answer 0

#[test]
fn test_decode_chunk_4_invalid_byte_at_end() {
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // Assuming 'A' (65) to 'Z' (90), 'a' (97) to 'z' (122), '0' (48) to '9' (57)
        for i in 0..=25 {
            table[65 + i] = i as u8; // A-Z => 0-25
        }
        for i in 0..=25 {
            table[97 + i] = (26 + i) as u8; // a-z => 26-51
        }
        for i in 0..=9 {
            table[48 + i] = (52 + i) as u8; // 0-9 => 52-61
        }
        table[43] = 62; // + => 62
        table[47] = 63; // / => 63
        table
    };
    let input: &[u8] = &[b'A', b'B', b'C', b'!',]; // '!' is invalid
    let index_at_start_of_input: usize = 0;
    let mut output = [0u8; 3];
    let result = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
    // result should be Err(DecodeError::InvalidByte(3, 33)); (33 is '!')
}

