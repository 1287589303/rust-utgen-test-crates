// Answer 0

#[test]
fn test_decode_chunk_8_invalid_byte_0() {
    let input: &[u8] = &[0xFF, 0xB0, 0xC0, 0xD0, 0xE0, 0xF0, 0xA0, 0xB0]; // Invalid first byte
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0xFF] = INVALID_VALUE; // Set this value to INVALID_VALUE
        table
    };
    let mut output = [0u8; 6];

    let result = decode_chunk_8(input, index_at_start_of_input, &decode_table, &mut output);
    
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input, input[0])));
}

#[test]
fn test_decode_chunk_8_invalid_byte_1() {
    let input: &[u8] = &[0x00, 0xFF, 0xC0, 0xD0, 0xE0, 0xF0, 0xA0, 0xB0]; // Invalid second byte
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0xFF] = INVALID_VALUE; // Set this value to INVALID_VALUE
        table
    };
    let mut output = [0u8; 6];

    let result = decode_chunk_8(input, index_at_start_of_input, &decode_table, &mut output);
    
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input + 1, input[1])));
}

#[test]
fn test_decode_chunk_8_invalid_byte_2() {
    let input: &[u8] = &[0x00, 0xB0, 0xFF, 0xD0, 0xE0, 0xF0, 0xA0, 0xB0]; // Invalid third byte
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0xFF] = INVALID_VALUE; // Set this value to INVALID_VALUE
        table
    };
    let mut output = [0u8; 6];

    let result = decode_chunk_8(input, index_at_start_of_input, &decode_table, &mut output);
    
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input + 2, input[2])));
}

#[test]
fn test_decode_chunk_8_invalid_byte_3() {
    let input: &[u8] = &[0x00, 0xB0, 0xC0, 0xFF, 0xE0, 0xF0, 0xA0, 0xB0]; // Invalid fourth byte
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0xFF] = INVALID_VALUE; // Set this value to INVALID_VALUE
        table
    };
    let mut output = [0u8; 6];

    let result = decode_chunk_8(input, index_at_start_of_input, &decode_table, &mut output);
    
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input + 3, input[3])));
}

#[test]
fn test_decode_chunk_8_invalid_byte_4() {
    let input: &[u8] = &[0x00, 0xB0, 0xC0, 0xD0, 0xFF, 0xF0, 0xA0, 0xB0]; // Invalid fifth byte
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0xFF] = INVALID_VALUE; // Set this value to INVALID_VALUE
        table
    };
    let mut output = [0u8; 6];

    let result = decode_chunk_8(input, index_at_start_of_input, &decode_table, &mut output);
    
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input + 4, input[4])));
}

#[test]
fn test_decode_chunk_8_invalid_byte_5() {
    let input: &[u8] = &[0x00, 0xB0, 0xC0, 0xD0, 0xE0, 0xFF, 0xA0, 0xB0]; // Invalid sixth byte
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0xFF] = INVALID_VALUE; // Set this value to INVALID_VALUE
        table
    };
    let mut output = [0u8; 6];

    let result = decode_chunk_8(input, index_at_start_of_input, &decode_table, &mut output);
    
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input + 5, input[5])));
}

#[test]
fn test_decode_chunk_8_invalid_byte_6() {
    let input: &[u8] = &[0x00, 0xB0, 0xC0, 0xD0, 0xE0, 0xF0, 0xFF, 0xB0]; // Invalid seventh byte
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0xFF] = INVALID_VALUE; // Set this value to INVALID_VALUE
        table
    };
    let mut output = [0u8; 6];

    let result = decode_chunk_8(input, index_at_start_of_input, &decode_table, &mut output);
    
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input + 6, input[6])));
}

#[test]
fn test_decode_chunk_8_invalid_byte_7() {
    let input: &[u8] = &[0x00, 0xB0, 0xC0, 0xD0, 0xE0, 0xF0, 0xA0, 0xFF]; // Invalid eighth byte
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0xFF] = INVALID_VALUE; // Set this value to INVALID_VALUE
        table
    };
    let mut output = [0u8; 6];

    let result = decode_chunk_8(input, index_at_start_of_input, &decode_table, &mut output);
    
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input + 7, input[7])));
}

