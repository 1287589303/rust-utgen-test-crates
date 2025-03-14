// Answer 0

#[test]
fn test_complete_quads_len_valid() {
    let input: &[u8] = b"QUJDRA==";
    let input_len_rem = 2;
    let output_len = 12; // enough for 3 complete quads (3 * 4 = 12)
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // Initialize with INVALID_VALUE
    decode_table[b'A' as usize] = 0; 
    decode_table[b'Q' as usize] = 1; 
    decode_table[b'J' as usize] = 2; 
    decode_table[b'D' as usize] = 3; 
    decode_table[b'R' as usize] = 4; 
    decode_table[b'A' as usize] = 0; 
    
    let result = complete_quads_len(input, input_len_rem, output_len, &decode_table);
    assert_eq!(result, Ok(4));
}

#[test]
fn test_complete_quads_len_too_small_output() {
    let input: &[u8] = b"QUJDRA==";
    let input_len_rem = 2;
    let output_len = 4; // not enough for 3 complete quads (3 * 4 = 12)
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; 
    decode_table[b'A' as usize] = 0; 
    decode_table[b'Q' as usize] = 1; 
    decode_table[b'J' as usize] = 2; 
    decode_table[b'D' as usize] = 3; 
    decode_table[b'R' as usize] = 4; 
    decode_table[b'A' as usize] = 0; 

    let result = complete_quads_len(input, input_len_rem, output_len, &decode_table);
    assert_eq!(result, Err(DecodeSliceError::OutputSliceTooSmall));
}

#[test]
fn test_complete_quads_len_invalid_last_byte() {
    let input: &[u8] = b"QUJDRA\xFF"; // last byte is invalid (not a pad byte)
    let input_len_rem = 1;
    let output_len = 12; 
    let mut decode_table: [u8; 256] = [INVALID_VALUE; 256]; 
    decode_table[b'A' as usize] = 0; 
    decode_table[b'Q' as usize] = 1; 
    decode_table[b'J' as usize] = 2; 
    decode_table[b'D' as usize] = 3; 
    decode_table[b'R' as usize] = 4; 

    let result = complete_quads_len(input, input_len_rem, output_len, &decode_table);
    assert!(result.is_err());
}

#[test]
fn test_complete_quads_len_empty_input() {
    let input: &[u8] = b""; // empty input
    let input_len_rem = 0;
    let output_len = 0; 
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; 

    let result = complete_quads_len(input, input_len_rem, output_len, &decode_table);
    assert_eq!(result, Ok(0)); // empty input should return 0
}

#[test]
fn test_complete_quads_len_edge_case() {
    let input: &[u8] = b"QUJDRA"; // 3 bytes with no padding
    let input_len_rem = 0; 
    let output_len = 6; // enough for 2 complete quads (2 * 4 = 8)
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; 
    decode_table[b'A' as usize] = 0; 
    decode_table[b'Q' as usize] = 1; 
    decode_table[b'J' as usize] = 2; 
    decode_table[b'D' as usize] = 3; 

    let result = complete_quads_len(input, input_len_rem, output_len, &decode_table);
    assert_eq!(result, Ok(4)); // should account for complete quads
}

