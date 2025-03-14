// Answer 0

#[test]
fn test_decode_suffix_invalid_byte() {
    const PAD_BYTE: u8 = b'=';
    const INVALID_VALUE: u8 = 0xFF; // Assuming 0xFF is used as INVALID_VALUE
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // All values are invalid
    let input: &[u8] = b"@@@@"; // Example invalid input
    let mut output: [u8; 4] = [0; 4];
    let input_index = 0;
    let output_index = 0;
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert!(result.is_err());

    if let Err(error) = result {
        match error {
            DecodeError::InvalidByte(index, byte) => {
                assert_eq!(index, input_index); // The index should match where the invalid byte was found
                assert_eq!(byte, b'@'); // The invalid byte is '@'
            },
            _ => panic!("Expected InvalidByte error, found another error type"),
        }
    }
}

#[test]
fn test_decode_suffix_invalid_byte_with_padding() {
    const PAD_BYTE: u8 = b'=';
    const INVALID_VALUE: u8 = 0xFF; // Assuming 0xFF is used as INVALID_VALUE
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // All values are invalid
    let input: &[u8] = b"AA=="; // Example input with padding, but invalid padding
    let mut output: [u8; 4] = [0; 4];
    let input_index = 0;
    let output_index = 0;
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
    
    assert!(result.is_err());

    if let Err(error) = result {
        match error {
            DecodeError::InvalidByte(index, byte) => {
                assert_eq!(index, input_index + 2); // The index should point to the padding byte
                assert_eq!(byte, PAD_BYTE); // The error is due to padding byte
            },
            _ => panic!("Expected InvalidByte error, found another error type"),
        }
    }
}

