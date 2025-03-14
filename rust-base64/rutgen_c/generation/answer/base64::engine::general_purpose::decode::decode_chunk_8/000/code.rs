// Answer 0

#[test]
fn test_decode_chunk_8_valid() {
    let input: [u8; 8] = [b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H'];
    let index_at_start_of_input = 0;
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
    let mut output: [u8; 6] = [0; 6];

    assert_eq!(decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output), Ok(()));
    assert_eq!(output, [0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_decode_chunk_8_invalid_byte() {
    let input: [u8; 8] = [b'A', b'B', b'Z', b'D', b'E', b'F', b'G', b'H'];
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'Z' as usize] = INVALID_VALUE; // Invalid byte
        table[b'D' as usize] = 3;
        table[b'E' as usize] = 4;
        table[b'F' as usize] = 5;
        table[b'G' as usize] = 6;
        table[b'H' as usize] = 7;
        table
    };
    let mut output: [u8; 6] = [0; 6];

    match decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output) {
        Err(DecodeError::InvalidByte(index, byte)) => {
            assert_eq!(index, 2);
            assert_eq!(byte, b'Z');
        },
        _ => panic!("Expected InvalidByte error"),
    }
}

#[test]
fn test_decode_chunk_8_out_of_bounds() {
    let input: [u8; 8] = [b'@'; 8]; // Out of bounds since '@' is not a valid Base64 character
    let index_at_start_of_input = 0;
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
    let mut output: [u8; 6] = [0; 6];

    match decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output) {
        Err(DecodeError::InvalidByte(index, byte)) => {
            assert_eq!(index, 0);
            assert_eq!(byte, b'@');
        },
        _ => panic!("Expected InvalidByte error"),
    }
}

