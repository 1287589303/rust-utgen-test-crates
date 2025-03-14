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
fn test_decode_chunk_8_valid_input() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    for (i, c) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
        decode_table[*c as usize] = i as u8; // Populate valid decode table values
    }
    let input: &[u8] = b"QUJDRA==";  // Corresponds to 'ABCD'
    let mut output = [0; 6];
    let result = decode_chunk_8(input, 0, &decode_table, &mut output);
    assert_eq!(result, Ok(()));
    assert_eq!(&output[..4], b"ABCD");
}

#[test]
fn test_decode_chunk_8_another_valid_input() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    for (i, c) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
        decode_table[*c as usize] = i as u8; // Populate valid decode table values
    }
    let input: &[u8] = b"U0VFTA==";  // Corresponds to 'SEET'
    let mut output = [0; 6];
    let result = decode_chunk_8(input, 0, &decode_table, &mut output);
    assert_eq!(result, Ok(()));
    assert_eq!(&output[..4], b"SEET");
}

#[test]
fn test_decode_chunk_8_exactly_6_bytes() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    for (i, c) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
        decode_table[*c as usize] = i as u8; // Populate valid decode table values
    }
    let input: &[u8] = b"QUNHRA==";  // Corresponds to 'AABC'
    let mut output = [0; 6];
    let result = decode_chunk_8(input, 0, &decode_table, &mut output);
    assert_eq!(result, Ok(()));
    assert_eq!(&output[..6], b"AABC\x00\x00");
}

