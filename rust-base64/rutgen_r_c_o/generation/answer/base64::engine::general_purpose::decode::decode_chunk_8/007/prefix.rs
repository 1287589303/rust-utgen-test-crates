// Answer 0

#[test]
fn test_decode_chunk_8_invalid_byte_at_sixth_index() {
    let input: [u8; 8] = [b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'@']; // '@' is invalid
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        table[b'E' as usize] = 4;
        table[b'F' as usize] = 5;
        table[b'G' as usize] = 6;
        table
    };
    let mut output: [u8; 6] = [0; 6];
    
    let _result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_8_invalid_byte_at_seventh_index() {
    let input: [u8; 8] = [b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'#']; // '#' is invalid
    let index_at_start_of_input: usize = 5;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        table[b'E' as usize] = 4;
        table[b'F' as usize] = 5;
        table[b'G' as usize] = 6;
        table
    };
    let mut output: [u8; 6] = [0; 6];
    
    let _result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
}

