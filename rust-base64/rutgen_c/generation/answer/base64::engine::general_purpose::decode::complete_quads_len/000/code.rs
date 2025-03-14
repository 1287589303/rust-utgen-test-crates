// Answer 0

#[test]
fn test_complete_quads_len_valid_case() {
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // Initialize valid base64 characters
        for (i, c) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
            table[*c as usize] = i as u8;
        }
        table[b'=' as usize] = 0; // padding character
        table
    };

    let input = b"QUJDRA=="; // "ABC" with padding
    let input_len_rem = input.len() % 4;
    let output_len = 3; // enough to decode "ABC"
    
    let result = complete_quads_len(input, input_len_rem, output_len, &decode_table);
    assert_eq!(result, Ok(4)); // The length of complete quads (ABC)
}

#[test]
fn test_complete_quads_len_invalid_byte() {
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // Initialize valid base64 characters
        for (i, c) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
            table[*c as usize] = i as u8;
        }
        table[b'=' as usize] = 0; // padding character
        table
    };

    let input = b"QUJDRA!\0"; // Invalid character '!'
    let input_len_rem = input.len() % 4;
    let output_len = 3;
    
    let result = complete_quads_len(input, input_len_rem, output_len, &decode_table);
    assert_eq!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(6, b'!'))));
}

#[test]
fn test_complete_quads_len_output_slice_too_small() {
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // Initialize valid base64 characters
        for (i, c) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
            table[*c as usize] = i as u8;
        }
        table[b'=' as usize] = 0; // padding character
        table
    };

    let input = b"QUJDRA=="; // "ABC" with padding
    let input_len_rem = input.len() % 4;
    let output_len = 2; // not enough to decode "ABC"
    
    let result = complete_quads_len(input, input_len_rem, output_len, &decode_table);
    assert_eq!(result, Err(DecodeSliceError::OutputSliceTooSmall));
} 

#[test]
fn test_complete_quads_len_no_padding() {
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // Initialize valid base64 characters
        for (i, c) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
            table[*c as usize] = i as u8;
        }
        table[b'=' as usize] = 0; // padding character
        table
    };

    let input = b"QUJD"; // "ABCD" without padding
    let input_len_rem = input.len() % 4;
    let output_len = 4; // enough for "ABCD"
    
    let result = complete_quads_len(input, input_len_rem, output_len, &decode_table);
    assert_eq!(result, Ok(4)); // The length of complete quads (ABCD)
}

