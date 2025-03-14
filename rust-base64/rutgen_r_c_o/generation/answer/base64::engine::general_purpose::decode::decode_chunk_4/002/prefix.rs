// Answer 0

#[test]
fn test_decode_chunk_4_invalid_byte_at_index_1() {
    let input: &[u8] = &[b'A', INVALID_VALUE, b'B', b'C'];
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0; // valid
        table[b'B' as usize] = 1; // valid
        table[b'C' as usize] = 2; // valid
        table
    };
    let mut output: [u8; 3] = [0; 3];

    let _result = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_4_invalid_byte_at_index_3() {
    let input: &[u8] = &[b'A', b'B', b'C', INVALID_VALUE];
    let index_at_start_of_input: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0; // valid
        table[b'B' as usize] = 1; // valid
        table[b'C' as usize] = 2; // valid
        table
    };
    let mut output: [u8; 3] = [0; 3];

    let _result = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
}

