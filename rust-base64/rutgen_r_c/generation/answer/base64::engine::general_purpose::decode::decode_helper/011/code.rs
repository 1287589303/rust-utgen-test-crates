// Answer 0

#[test]
fn test_decode_helper_empty_input() {
    let input: &[u8] = &[];
    let mut output: [u8; 10] = [0; 10];
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 0 };
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    
    let result = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::Indifferent);
    
    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.decoded_len, 0);
    assert_eq!(metadata.padding_offset, None);
}

#[test]
fn test_decode_helper_nonempty_input_no_chunks() {
    let input: &[u8] = &[b'A', b'B', b'C', b'D']; // Example of valid base64 input
    let mut output: [u8; 10] = [0; 10];
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 6 }; // Expecting 6 bytes output
    let decode_table: [u8; 256] = { 
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0; // base64 for A = 0
        table[b'B' as usize] = 1; // base64 for B = 1
        table[b'C' as usize] = 2; // base64 for C = 2
        table[b'D' as usize] = 3; // base64 for D = 3
        table
    };

    let result = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::Indifferent);
    
    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.decoded_len, 3); // Expecting decoded length to be 3 (ABC -> 3 bytes)
    assert_eq!(metadata.padding_offset, None); // No padding in this case
} 

#[test]
fn test_decode_helper_nonempty_input_only_complete_quads() {
    let input: &[u8] = &[b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H']; // More valid base64 input
    let mut output: [u8; 10] = [0; 10];
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 9 }; // Expecting 9 bytes output
    let decode_table: [u8; 256] = { 
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        table[b'E' as usize] = 4;
        table[b'F' as usize] = 5;
        table[b'G' as usize] = 6;
        table[b'H' as usize] = 7;
        table
    };

    let result = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::Indifferent);
    
    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.decoded_len, 6); // Expecting decoded length to reflect the valid output
    assert_eq!(metadata.padding_offset, None); // No padding
}

