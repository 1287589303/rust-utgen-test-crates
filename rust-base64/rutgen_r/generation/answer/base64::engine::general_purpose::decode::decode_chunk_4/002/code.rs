// Answer 0

#[derive(Debug)]
struct DecodeError {
    index: usize,
    byte: u8,
}

impl DecodeError {
    fn invalid_byte(index: usize, byte: u8) -> Self {
        DecodeError { index, byte }
    }
}

const INVALID_VALUE: u8 = 255; // Assuming INVALID_VALUE is defined as 255 for invalid bytes

fn decode_chunk_4(
    input: &[u8],
    index_at_start_of_input: usize,
    decode_table: &[u8; 256],
    output: &mut [u8],
) -> Result<(), DecodeError> {
    let morsel = decode_table[usize::from(input[0])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::invalid_byte(index_at_start_of_input, input[0]));
    }
    let mut accum = u32::from(morsel) << 26;

    let morsel = decode_table[usize::from(input[1])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::invalid_byte(
            index_at_start_of_input + 1,
            input[1],
        ));
    }
    accum |= u32::from(morsel) << 20;

    let morsel = decode_table[usize::from(input[2])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::invalid_byte(
            index_at_start_of_input + 2,
            input[2],
        ));
    }
    accum |= u32::from(morsel) << 14;

    let morsel = decode_table[usize::from(input[3])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::invalid_byte(
            index_at_start_of_input + 3,
            input[3],
        ));
    }
    accum |= u32::from(morsel) << 8;

    output[..3].copy_from_slice(&accum.to_be_bytes()[..3]);

    Ok(())
}

#[test]
fn test_decode_chunk_4_invalid_byte_at_second_position() {
    let decode_table: [u8; 256] = [0; 256];
    decode_table[b'A' as usize] = 0; // Valid mapping for 'A'
    decode_table[b'B' as usize] = 1; // Valid mapping for 'B'
    decode_table[b'C' as usize] = 2; // Valid mapping for 'C'
    // Input 'D' will be invalid
    let input = [b'A', b'B', b'C', b'D'];
    let index_at_start_of_input = 0;
    let mut output = [0u8; 3];

    let result = decode_chunk_4(&input, index_at_start_of_input, &decode_table, &mut output);

    assert_eq!(
        result,
        Err(DecodeError::invalid_byte(index_at_start_of_input + 1, b'D'))
    );
}

