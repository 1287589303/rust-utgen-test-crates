// Answer 0

#[test]
fn test_decode_chunk_4_valid_case() {
    let input: &[u8] = b"QUJD"; // Valid base64 input corresponding to "ABC"
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'Q' as usize] = 16;
        table[b'J' as usize] = 9;
        table[b'D' as usize] = 3;
        table
    };
    let mut output: [u8; 3] = [0; 3];
    
    let _ = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_4_another_valid_case() {
    let input: &[u8] = b"YWJj"; // Valid base64 input corresponding to "abc"
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'a' as usize] = 26;
        table[b'Y' as usize] = 24;
        table[b'J' as usize] = 9;
        table[b'c' as usize] = 28;
        table
    };
    let mut output: [u8; 3] = [0; 3];
    
    let _ = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
}

