// Answer 0

#[test]
fn test_decode_chunk_8_invalid_byte_first() {
    // Arrange
    let input: [u8; 8] = [255, b'A', b'B', b'C', b'D', b'E', b'F', b'G']; // 255 is invalid
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let mut output: [u8; 6] = [0; 6];

    // Act
    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);

    // Assert
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input, input[0])));
}

#[test]
fn test_decode_chunk_8_invalid_byte_second() {
    // Arrange
    let input: [u8; 8] = [b'A', 255, b'B', b'C', b'D', b'E', b'F', b'G']; // 255 is invalid
    let index_at_start_of_input = 1;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let mut output: [u8; 6] = [0; 6];

    // Act
    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);

    // Assert
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input, input[1])));
}

#[test]
fn test_decode_chunk_8_invalid_byte_third() {
    // Arrange
    let input: [u8; 8] = [b'A', b'B', 255, b'C', b'D', b'E', b'F', b'G']; // 255 is invalid
    let index_at_start_of_input = 2;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let mut output: [u8; 6] = [0; 6];

    // Act
    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);

    // Assert
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input, input[2])));
}

#[test]
fn test_decode_chunk_8_invalid_byte_fourth() {
    // Arrange
    let input: [u8; 8] = [b'A', b'B', b'C', 255, b'D', b'E', b'F', b'G']; // 255 is invalid
    let index_at_start_of_input = 3;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let mut output: [u8; 6] = [0; 6];

    // Act
    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);

    // Assert
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input, input[3])));
}

#[test]
fn test_decode_chunk_8_invalid_byte_fifth() {
    // Arrange
    let input: [u8; 8] = [b'A', b'B', b'C', b'D', 255, b'E', b'F', b'G']; // 255 is invalid
    let index_at_start_of_input = 4;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let mut output: [u8; 6] = [0; 6];

    // Act
    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);

    // Assert
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input, input[4])));
}

#[test]
fn test_decode_chunk_8_invalid_byte_sixth() {
    // Arrange
    let input: [u8; 8] = [b'A', b'B', b'C', b'D', b'E', 255, b'F', b'G']; // 255 is invalid
    let index_at_start_of_input = 5;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let mut output: [u8; 6] = [0; 6];

    // Act
    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);

    // Assert
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input, input[5])));
}

#[test]
fn test_decode_chunk_8_invalid_byte_seventh() {
    // Arrange
    let input: [u8; 8] = [b'A', b'B', b'C', b'D', b'E', b'F', 255, b'G']; // 255 is invalid
    let index_at_start_of_input = 6;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let mut output: [u8; 6] = [0; 6];

    // Act
    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);

    // Assert
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input, input[6])));
}

#[test]
fn test_decode_chunk_8_invalid_byte_eighth() {
    // Arrange
    let input: [u8; 8] = [b'A', b'B', b'C', b'D', b'E', b'F', b'G', 255]; // 255 is invalid
    let index_at_start_of_input = 7;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let mut output: [u8; 6] = [0; 6];

    // Act
    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);

    // Assert
    assert_eq!(result, Err(DecodeError::InvalidByte(index_at_start_of_input, input[7])));
}

