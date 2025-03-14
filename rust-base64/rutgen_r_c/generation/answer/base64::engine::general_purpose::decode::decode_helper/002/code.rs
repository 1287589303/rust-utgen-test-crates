// Answer 0

#[test]
fn test_decode_helper_valid_case() {
    #[derive(Debug)]
    struct TestEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }
    let estimate = TestEstimate { rem: 0, conservative_decoded_len: 24 };

    let input: &[u8] = &[
        // Base64 to decode (example: "TWFu")
        0x54, 0x57, 0xF9, 0xE5, // This input encodes to "Man"
    ];

    let mut output: [u8; 24] = [0; 24];
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'M' as usize] = 0;
        table[b'a' as usize] = 1;
        table[b'n' as usize] = 2;
        table[b'=' as usize] = 3; // assuming it is part of the example
        table
    };
    let allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result: Result<DecodeMetadata, DecodeSliceError> = decode_helper(
        input,
        &estimate,
        &mut output,
        &decode_table,
        allow_trailing_bits,
        padding_mode,
    );

    assert!(result.is_ok());
}

#[test]
fn test_decode_helper_invalid_byte() {
    #[derive(Debug)]
    struct TestEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }
    let estimate = TestEstimate { rem: 0, conservative_decoded_len: 0 };

    let input: &[u8] = &[
        // Invalid byte (e.g., an invalid base64 character)
        0x54, 0x57, 0xFF, 0xE5, // The byte 0xFF is invalid
    ];

    let mut output: [u8; 24] = [0; 24];
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'M' as usize] = 0;
        table[b'a' as usize] = 1;
        table[b'n' as usize] = 2;
        table
    };
    let allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result: Result<DecodeMetadata, DecodeSliceError> = decode_helper(
        input,
        &estimate,
        &mut output,
        &decode_table,
        allow_trailing_bits,
        padding_mode,
    );

    assert!(result.is_err());
    if let Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(idx, byte))) = result {
        assert_eq!(idx, 2);
        assert_eq!(byte, 0xFF);
    } else {
        panic!("Unexpected result");
    }
}

#[test]
fn test_decode_helper_invalid_padding() {
    #[derive(Debug)]
    struct TestEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }
    let estimate = TestEstimate { rem: 1, conservative_decoded_len: 0 };

    let input: &[u8] = &[
        0x54, 0x57, 0x41, 0xE5, // The 'E5' here should have a padding
    ];

    let mut output: [u8; 24] = [0; 24];
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'M' as usize] = 0;
        table[b'a' as usize] = 1;
        table[b'n' as usize] = 2;
        table
    };
    let allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result: Result<DecodeMetadata, DecodeSliceError> = decode_helper(
        input,
        &estimate,
        &mut output,
        &decode_table,
        allow_trailing_bits,
        padding_mode,
    );

    assert!(result.is_err());
    if let Err(DecodeSliceError::DecodeError(DecodeError::InvalidPadding)) = result {
        // expected error
    } else {
        panic!("Unexpected result");
    }
}

