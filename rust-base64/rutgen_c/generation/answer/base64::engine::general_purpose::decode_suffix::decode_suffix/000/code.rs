// Answer 0

#[test]
fn test_decode_suffix_valid_case() {
    let input: &[u8] = b"c3"; // valid base64 representation of 'c'
    let input_index = 0;
    let mut output = [0u8; 1];
    let output_index = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'a' as usize] = 0; // 0
        table[b'b' as usize] = 1; // 1
        table[b'c' as usize] = 2; // 2
        table[b'd' as usize] = 3; // 3
        table[b'e' as usize] = 4; // 4
        table[b'f' as usize] = 5; // 5
        table[b'g' as usize] = 6; // 6
        table[b'h' as usize] = 7; // 7
        table[b'i' as usize] = 8; // 8
        table[b'j' as usize] = 9; // 9
        table[b'k' as usize] = 10; // 10
        table[b'l' as usize] = 11; // 11
        table[b'm' as usize] = 12; // 12
        table[b'n' as usize] = 13; // 13
        table[b'o' as usize] = 14; // 14
        table[b'p' as usize] = 15; // 15
        table[b'q' as usize] = 16; // 16
        table[b'r' as usize] = 17; // 17
        table[b's' as usize] = 18; // 18
        table[b't' as usize] = 19; // 19
        table[b'u' as usize] = 20; // 20
        table[b'v' as usize] = 21; // 21
        table[b'w' as usize] = 22; // 22
        table[b'x' as usize] = 23; // 23
        table[b'y' as usize] = 24; // 24
        table[b'z' as usize] = 25; // 25
        table[b'A' as usize] = 26; // 26
        table[b'B' as usize] = 27; // 27
        table[b'C' as usize] = 28; // 28
        table[b'D' as usize] = 29; // 29
        table[b'E' as usize] = 30; // 30
        table[b'F' as usize] = 31; // 31
        table[b'G' as usize] = 32; // 32
        table[b'H' as usize] = 33; // 33
        table[b'I' as usize] = 34; // 34
        table[b'J' as usize] = 35; // 35
        table[b'K' as usize] = 36; // 36
        table[b'L' as usize] = 37; // 37
        table[b'M' as usize] = 38; // 38
        table[b'N' as usize] = 39; // 39
        table[b'O' as usize] = 40; // 40
        table[b'P' as usize] = 41; // 41
        table[b'Q' as usize] = 42; // 42
        table[b'R' as usize] = 43; // 43
        table[b'S' as usize] = 44; // 44
        table[b'T' as usize] = 45; // 45
        table[b'U' as usize] = 46; // 46
        table[b'V' as usize] = 47; // 47
        table[b'W' as usize] = 48; // 48
        table[b'X' as usize] = 49; // 49
        table[b'Y' as usize] = 50; // 50
        table[b'Z' as usize] = 51; // 51
        table[b'0' as usize] = 52; // 52
        table[b'1' as usize] = 53; // 53
        table[b'2' as usize] = 54; // 54
        table[b'3' as usize] = 55; // 55
        table[b'4' as usize] = 56; // 56
        table[b'5' as usize] = 57; // 57
        table[b'6' as usize] = 58; // 58
        table[b'7' as usize] = 59; // 59
        table[b'8' as usize] = 60; // 60
        table[b'9' as usize] = 61; // 61
        table[b'+' as usize] = 62; // 62
        table[b'/' as usize] = 63; // 63
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    assert!(result.is_ok());

    let metadata = result.unwrap();
    assert_eq!(metadata.decoded_len, 1);
    assert_eq!(metadata.padding_offset, None);
    assert_eq!(output[0], b'c');
}

#[test]
fn test_decode_suffix_invalid_byte() {
    let input: &[u8] = b"c@"; // '@' is not a valid base64 character
    let input_index = 0;
    let mut output = [0u8; 1];
    let output_index = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    assert!(result.is_err());
    match result.unwrap_err() {
        DecodeSliceError::DecodeError(DecodeError::InvalidByte(offset, b)) => {
            assert_eq!(offset, 1); // Error at the index of '@'
            assert_eq!(b, b'@');
        }
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_decode_suffix_invalid_padding() {
    let input: &[u8] = b"c3=="; // Invalid padding for base64
    let input_index = 0;
    let mut output = [0u8; 1];
    let output_index = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'a' as usize] = 0; // 0
        // ...
        table[b'c' as usize] = 2; // 2
        // ...
        table[b'=' as usize] = INVALID_VALUE; // Padding character
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), DecodeSliceError::DecodeError(DecodeError::InvalidPadding));
}

#[test]
fn test_decode_suffix_output_too_small() {
    let input: &[u8] = b"c3"; 
    let input_index = 0;
    let mut output = [0u8; 0]; // Output buffer too small
    let output_index = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'a' as usize] = 0; // 0
        // ...
        table[b'c' as usize] = 2; // 2
        // ...
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    assert_eq!(result.unwrap_err(), DecodeSliceError::OutputSliceTooSmall);
}

