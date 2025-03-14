// Answer 0

#[test]
fn test_decode_chunk_8_invalid_byte_first_element() {
    let input: [u8; 8] = [0xFF, b'A', b'B', b'C', b'D', b'E', b'F', b'G'];
    let index_at_start_of_input = 0;
    let decode_table: [u8; 256] = {
        let mut table = [0; 256];
        table[0xFF] = INVALID_VALUE;
        table[b'A' as usize] = 0; // valid
        table[b'B' as usize] = 1; // valid
        table[b'C' as usize] = 2; // valid
        table[b'D' as usize] = 3; // valid
        table[b'E' as usize] = 4; // valid
        table[b'F' as usize] = 5; // valid
        table[b'G' as usize] = 6; // valid
        table
    };
    let mut output: [u8; 6] = [0; 6];
    let _result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_8_invalid_byte_second_element() {
    let input: [u8; 8] = [b'A', 0xFF, b'C', b'D', b'E', b'F', b'G', b'H'];
    let index_at_start_of_input = 1;
    let decode_table: [u8; 256] = {
        let mut table = [0; 256];
        table[0xFF] = INVALID_VALUE;
        table[b'A' as usize] = 0; // valid
        table[b'C' as usize] = 2; // valid
        table[b'D' as usize] = 3; // valid
        table[b'E' as usize] = 4; // valid
        table[b'F' as usize] = 5; // valid
        table[b'G' as usize] = 6; // valid
        table[b'H' as usize] = 7; // valid
        table
    };
    let mut output: [u8; 6] = [0; 6];
    let _result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_8_invalid_byte_third_element() {
    let input: [u8; 8] = [b'A', b'B', 0xFF, b'D', b'E', b'F', b'G', b'H'];
    let index_at_start_of_input = 2;
    let decode_table: [u8; 256] = {
        let mut table = [0; 256];
        table[0xFF] = INVALID_VALUE;
        table[b'A' as usize] = 0; // valid
        table[b'B' as usize] = 1; // valid
        table[b'D' as usize] = 3; // valid
        table[b'E' as usize] = 4; // valid
        table[b'F' as usize] = 5; // valid
        table[b'G' as usize] = 6; // valid
        table[b'H' as usize] = 7; // valid
        table
    };
    let mut output: [u8; 6] = [0; 6];
    let _result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_8_invalid_byte_fourth_element() {
    let input: [u8; 8] = [b'A', b'B', b'C', 0xFF, b'E', b'F', b'G', b'H'];
    let index_at_start_of_input = 3;
    let decode_table: [u8; 256] = {
        let mut table = [0; 256];
        table[0xFF] = INVALID_VALUE;
        table[b'A' as usize] = 0; // valid
        table[b'B' as usize] = 1; // valid
        table[b'C' as usize] = 2; // valid
        table[b'E' as usize] = 4; // valid
        table[b'F' as usize] = 5; // valid
        table[b'G' as usize] = 6; // valid
        table[b'H' as usize] = 7; // valid
        table
    };
    let mut output: [u8; 6] = [0; 6];
    let _result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_8_invalid_byte_fifth_element() {
    let input: [u8; 8] = [b'A', b'B', b'C', b'D', 0xFF, b'F', b'G', b'H'];
    let index_at_start_of_input = 4;
    let decode_table: [u8; 256] = {
        let mut table = [0; 256];
        table[0xFF] = INVALID_VALUE;
        table[b'A' as usize] = 0; // valid
        table[b'B' as usize] = 1; // valid
        table[b'C' as usize] = 2; // valid
        table[b'D' as usize] = 3; // valid
        table[b'F' as usize] = 5; // valid
        table[b'G' as usize] = 6; // valid
        table[b'H' as usize] = 7; // valid
        table
    };
    let mut output: [u8; 6] = [0; 6];
    let _result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_8_invalid_byte_sixth_element() {
    let input: [u8; 8] = [b'A', b'B', b'C', b'D', b'E', 0xFF, b'G', b'H'];
    let index_at_start_of_input = 5;
    let decode_table: [u8; 256] = {
        let mut table = [0; 256];
        table[0xFF] = INVALID_VALUE;
        table[b'A' as usize] = 0; // valid
        table[b'B' as usize] = 1; // valid
        table[b'C' as usize] = 2; // valid
        table[b'D' as usize] = 3; // valid
        table[b'E' as usize] = 4; // valid
        table[b'G' as usize] = 6; // valid
        table[b'H' as usize] = 7; // valid
        table
    };
    let mut output: [u8; 6] = [0; 6];
    let _result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
}

#[test]
fn test_decode_chunk_8_invalid_byte_seventh_element() {
    let input: [u8; 8] = [b'A', b'B', b'C', b'D', b'E', b'F', 0xFF, b'H'];
    let index_at_start_of_input = 6;
    let decode_table: [u8; 256] = {
        let mut table = [0; 256];
        table[0xFF] = INVALID_VALUE;
        table[b'A' as usize] = 0; // valid
        table[b'B' as usize] = 1; // valid
        table[b'C' as usize] = 2; // valid
        table[b'D' as usize] = 3; // valid
        table[b'E' as usize] = 4; // valid
        table[b'F' as usize] = 5; // valid
        table[b'H' as usize] = 7; // valid
        table
    };
    let mut output: [u8; 6] = [0; 6];
    let _result = decode_chunk_8(&input, index_at_start_of_input, &decode_table, &mut output);
}

