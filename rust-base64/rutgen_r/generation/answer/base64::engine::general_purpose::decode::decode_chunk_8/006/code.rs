// Answer 0

#[derive(Debug)]
struct DecodeError {
    index: usize,
    byte: u8,
}

const INVALID_VALUE: u8 = 255;

fn decode_chunk_8(
    input: &[u8],
    index_at_start_of_input: usize,
    decode_table: &[u8; 256],
    output: &mut [u8],
) -> Result<(), DecodeError> {
    let morsel = decode_table[usize::from(input[0])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError { index: index_at_start_of_input, byte: input[0] });
    }
    let mut accum = u64::from(morsel) << 58;

    let morsel = decode_table[usize::from(input[1])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError { index: index_at_start_of_input + 1, byte: input[1] });
    }
    accum |= u64::from(morsel) << 52;

    let morsel = decode_table[usize::from(input[2])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError { index: index_at_start_of_input + 2, byte: input[2] });
    }
    accum |= u64::from(morsel) << 46;

    let morsel = decode_table[usize::from(input[3])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError { index: index_at_start_of_input + 3, byte: input[3] });
    }
    accum |= u64::from(morsel) << 40;

    let morsel = decode_table[usize::from(input[4])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError { index: index_at_start_of_input + 4, byte: input[4] });
    }
    accum |= u64::from(morsel) << 34;

    let morsel = decode_table[usize::from(input[5])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError { index: index_at_start_of_input + 5, byte: input[5] });
    }
    accum |= u64::from(morsel) << 28;

    let morsel = decode_table[usize::from(input[6])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError { index: index_at_start_of_input + 6, byte: input[6] });
    }
    accum |= u64::from(morsel) << 22;

    let morsel = decode_table[usize::from(input[7])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError { index: index_at_start_of_input + 7, byte: input[7] });
    }
    accum |= u64::from(morsel) << 16;

    output[..6].copy_from_slice(&accum.to_be_bytes()[..6]);
    
    Ok(())
}

#[test]
fn test_decode_chunk_8_invalid_byte() {
    let input: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [0; 256];
        table[0] = 0; // valid
        table[1] = 1; // valid
        table[2] = 2; // valid
        table[3] = 3; // valid
        table[4] = 4; // valid
        table[5] = 5; // valid
        table[6] = 6; // valid
        table[7] = 7; // valid
        for i in 8..256 {
            table[i] = INVALID_VALUE; // invalid
        }
        table
    };
    let mut output: [u8; 6] = [0; 6];

    let result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
    
    assert!(result.is_err());
    if let Err(DecodeError { index, byte }) = result {
        assert_eq!(index, 5);
        assert_eq!(byte, 5);
    }
}

