// Answer 0

#[test]
fn test_decode_helper_valid_case() {
    let input: &[u8] = b"SGVsbG8gV29ybGQ="; // "Hello World" in base64
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 11 };
    let mut output = vec![0u8; 11]; // Expected output length for "Hello World"
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for (i, &c) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
            table[c as usize] = i as u8;
        }
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_helper(input, &estimate, &mut output, &decode_table, decode_allow_trailing_bits, padding_mode);
    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.decoded_len, 11);
    assert!(metadata.padding_offset.is_none());
}

#[test]
#[should_panic]
fn test_decode_helper_invalid_case() {
    let input: &[u8] = b"SGVsbG8gV29ybGQ="; // "Hello World" in base64
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 10 }; // Wrong estimate
    let mut output = vec![0u8; 10]; // Smaller than required output length
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for (i, &c) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
            table[c as usize] = i as u8;
        }
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let _result = decode_helper(input, &estimate, &mut output, &decode_table, decode_allow_trailing_bits, padding_mode);
}

