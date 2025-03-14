// Answer 0

#[test]
fn test_decode_suffix_with_invalid_padding_case_1() {
    let input: &[u8] = b"BA=="; // Padding with invalid characters after it
    let input_index = 0;
    let mut output = [0u8; 3];
    let output_index = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'B' as usize] = 1; // B -> 1
        table[b'A' as usize] = 0; // A -> 0
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert_eq!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(2, b'='))));
}

#[test]
fn test_decode_suffix_with_invalid_padding_case_2() {
    let input: &[u8] = b"BAA="; // Padding after one character in the current quad
    let input_index = 0;
    let mut output = [0u8; 3];
    let output_index = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'B' as usize] = 1; // B -> 1
        table[b'A' as usize] = 0; // A -> 0
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert_eq!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(2, b'='))));
}

#[test]
fn test_decode_suffix_with_invalid_padding_case_3() {
    let input: &[u8] = b"B=="; // More than two characters of padding
    let input_index = 0;
    let mut output = [0u8; 3];
    let output_index = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'B' as usize] = 1; // B -> 1
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert_eq!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(1, b'='))));
}

#[test]
fn test_decode_suffix_with_invalid_padding_case_4() {
    let input: &[u8] = b"BAA"; // Test case with invalid length
    let input_index = 0;
    let mut output = [0u8; 3];
    let output_index = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'B' as usize] = 1; // B -> 1
        table[b'A' as usize] = 0; // A -> 0
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert_eq!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidLength(3))));
}

