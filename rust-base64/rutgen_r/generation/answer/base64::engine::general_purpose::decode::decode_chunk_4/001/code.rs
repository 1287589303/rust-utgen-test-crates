// Answer 0

#[test]
fn test_decode_chunk_4_invalid_byte_first() {
    let input: &[u8] = &[255, 0, 0, 0]; // 255 is an invalid index for the decode_table
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // All values are INVALID_VALUE
    let mut output = [0u8; 3];

    let result = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
    assert!(result.is_err());
    if let Err(DecodeError::InvalidByte(index, byte)) = result {
        assert_eq!(index, 0);
        assert_eq!(byte, 255);
    } else {
        panic!("Expected Err with InvalidByte");
    }
}

#[test]
fn test_decode_chunk_4_invalid_byte_second() {
    let input: &[u8] = &[0, 255, 0, 0]; // 255 is an invalid index for the decode_table
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // All values are INVALID_VALUE
    let mut output = [0u8; 3];

    let result = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
    assert!(result.is_err());
    if let Err(DecodeError::InvalidByte(index, byte)) = result {
        assert_eq!(index, 1);
        assert_eq!(byte, 255);
    } else {
        panic!("Expected Err with InvalidByte");
    }
}

#[test]
fn test_decode_chunk_4_invalid_byte_third() {
    let input: &[u8] = &[0, 0, 255, 0]; // 255 is an invalid index for the decode_table
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // All values are INVALID_VALUE
    let mut output = [0u8; 3];

    let result = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
    assert!(result.is_err());
    if let Err(DecodeError::InvalidByte(index, byte)) = result {
        assert_eq!(index, 2);
        assert_eq!(byte, 255);
    } else {
        panic!("Expected Err with InvalidByte");
    }
}

#[test]
fn test_decode_chunk_4_invalid_byte_fourth() {
    let input: &[u8] = &[0, 0, 0, 255]; // 255 is an invalid index for the decode_table
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // All values are INVALID_VALUE
    let mut output = [0u8; 3];

    let result = decode_chunk_4(input, index_at_start_of_input, &decode_table, &mut output);
    assert!(result.is_err());
    if let Err(DecodeError::InvalidByte(index, byte)) = result {
        assert_eq!(index, 3);
        assert_eq!(byte, 255);
    } else {
        panic!("Expected Err with InvalidByte");
    }
}

