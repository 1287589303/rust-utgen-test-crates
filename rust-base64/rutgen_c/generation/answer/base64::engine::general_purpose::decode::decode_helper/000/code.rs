// Answer 0

#[test]
fn test_decode_helper_valid() {
    let input: &[u8] = b"SGVsbG8gd29ybGQ="; // "Hello world" in base64
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 11 };
    let mut output = vec![0u8; 11]; // "Hello world" length
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for (i, &b) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
            table[b as usize] = i as u8;
        }
        table[b'=' as usize] = PAD_BYTE;
        table
    };
    let result = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::RequireCanonical);
    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.decoded_len, 11);
    assert_eq!(metadata.padding_offset, Some(15)); // position of '='
}

#[test]
fn test_decode_helper_invalid_byte() {
    let input: &[u8] = b"SGVsbG8gd29ybGQ#"; // Invalid character '#'
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 11 };
    let mut output = vec![0u8; 11];
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for (i, &b) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
            table[b as usize] = i as u8;
        }
        table[b'=' as usize] = PAD_BYTE;
        table
    };
    let result = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::RequireCanonical);
    assert!(result.is_err());
    if let Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(index, byte))) = result {
        assert_eq!(index, 15);
        assert_eq!(byte, b'#');
    } else {
        panic!("Expected DecodeError::InvalidByte");
    }
}

#[test]
fn test_decode_helper_insufficient_output_space() {
    let input: &[u8] = b"SGVsbG8gd29ybGQ="; // "Hello world" in base64
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 11 };
    let mut output = vec![0u8; 5]; // Insufficient length
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for (i, &b) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
            table[b as usize] = i as u8;
        }
        table[b'=' as usize] = PAD_BYTE;
        table
    };
    let result = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::RequireCanonical);
    assert!(result.is_err());
    assert_eq!(result, Err(DecodeSliceError::OutputSliceTooSmall));
}

