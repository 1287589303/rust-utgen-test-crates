// Answer 0

#[test]
fn test_decode_helper_valid_input() {
    struct GeneralPurposeEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let input: &[u8] = b"SGVsbG8sIFdvcmxkIQ=="; // "Hello, World!" in base64
    let mut output = vec![0u8; 16]; // sufficient size for decoded output
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for (i, c) in b"AZaz09+/".iter().enumerate() {
            table[*c as usize] = i as u8; // fill with base64 mapping
        }
        table
    };
    let estimate = GeneralPurposeEstimate {
        rem: 0,
        conservative_decoded_len: 16,
    };
    let result = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::RequireCanonical);
    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.decoded_len, 13); // "Hello, World!" has 13 characters
    assert_eq!(metadata.padding_offset, Some(16));
}

#[test]
fn test_decode_helper_invalid_byte() {
    struct GeneralPurposeEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let input: &[u8] = b"SGVsbG8sIFdvcmxkIQ$=="; // Invalid base64 input with `$`
    let mut output = vec![0u8; 16];
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for (i, c) in b"AZaz09+/".iter().enumerate() {
            table[*c as usize] = i as u8;
        }
        table
    };
    let estimate = GeneralPurposeEstimate {
        rem: 0,
        conservative_decoded_len: 16,
    };
    let result = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::RequireCanonical);
    assert!(result.is_err());
    if let Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(index, byte))) = result {
        assert_eq!(index, 23); // Position of invalid byte `$`
        assert_eq!(byte, b'$');
    } else {
        panic!("Expected DecodeError::InvalidByte");
    }
}

#[test]
fn test_decode_helper_too_small_output_slice() {
    struct GeneralPurposeEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let input: &[u8] = b"SGVsbG8sIFdvcmxkIQ=="; // "Hello, World!" in base64
    let mut output = vec![0u8; 5]; // too small for the output
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for (i, c) in b"AZaz09+/".iter().enumerate() {
            table[*c as usize] = i as u8;
        }
        table
    };
    let estimate = GeneralPurposeEstimate {
        rem: 0,
        conservative_decoded_len: 16,
    };

    let result = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::RequireCanonical);
    assert!(result.is_err());
    assert_eq!(result, Err(DecodeSliceError::OutputSliceTooSmall));
}

#[test]
fn test_decode_helper_invalid_length() {
    struct GeneralPurposeEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let input: &[u8] = b"SGVsbG8sIFdvcmxkIQ==O"; // Invalid base64 length with extra byte
    let mut output = vec![0u8; 16];
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for (i, c) in b"AZaz09+/".iter().enumerate() {
            table[*c as usize] = i as u8;
        }
        table
    };
    let estimate = GeneralPurposeEstimate {
        rem: 0,
        conservative_decoded_len: 16,
    };

    let result = decode_helper(input, &estimate, &mut output, &decode_table, false, DecodePaddingMode::RequireCanonical);
    assert!(result.is_err());
    if let Err(DecodeSliceError::DecodeError(DecodeError::InvalidLength(index))) = result {
        assert_eq!(index, 23); // The position of the invalid length
    } else {
        panic!("Expected DecodeError::InvalidLength");
    }
}

