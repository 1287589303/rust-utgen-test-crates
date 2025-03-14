// Answer 0

#[derive(Debug)]
struct DecodeError {
    index: usize,
    byte: u8,
}

const INVALID_VALUE: u8 = 255;

fn decode_chunk_4(
    input: &[u8],
    index_at_start_of_input: usize,
    decode_table: &[u8; 256],
    output: &mut [u8],
) -> Result<(), DecodeError> {
    let morsel = decode_table[usize::from(input[0])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(index_at_start_of_input, input[0]));
    }
    let mut accum = u32::from(morsel) << 26;

    let morsel = decode_table[usize::from(input[1])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(index_at_start_of_input + 1, input[1]));
    }
    accum |= u32::from(morsel) << 20;

    let morsel = decode_table[usize::from(input[2])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(index_at_start_of_input + 2, input[2]));
    }
    accum |= u32::from(morsel) << 14;

    let morsel = decode_table[usize::from(input[3])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(index_at_start_of_input + 3, input[3]));
    }
    accum |= u32::from(morsel) << 8;

    output[..3].copy_from_slice(&accum.to_be_bytes()[..3]);

    Ok(())
}

#[test]
fn test_decode_chunk_4_valid_input() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let valid_indexes = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    for (i, &byte) in valid_indexes.iter().enumerate() {
        decode_table[byte as usize] = i as u8;
    }

    let input = b"QUF";
    let mut output = [0u8; 3];
    let result = decode_chunk_4(input, 0, &decode_table, &mut output);
    assert!(result.is_ok());
    assert_eq!(output, b"AAE");
}

#[test]
fn test_decode_chunk_4_invalid_byte() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    for (i, byte) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
        decode_table[*byte as usize] = i as u8;
    }

    let input = b"QU@"; // Invalid byte '@'
    let mut output = [0u8; 3];
    let result = decode_chunk_4(input, 0, &decode_table, &mut output);
    assert!(result.is_err());
    if let Err(DecodeError { index, byte }) = result {
        assert_eq!(index, 2);
        assert_eq!(byte, b'@');
    }
}

#[test]
fn test_decode_chunk_4_partial_output() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let valid_indexes = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    for (i, &byte) in valid_indexes.iter().enumerate() {
        decode_table[byte as usize] = i as u8;
    }

    let input = b"QUF";
    let mut output = [0u8; 3];
    let result = decode_chunk_4(input, 0, &decode_table, &mut output);
    assert!(result.is_ok());
    assert_eq!(output, b"AAE");
}

