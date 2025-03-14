// Answer 0

#[test]
fn test_decode_helper_valid_chunk_processing() {
    struct GeneralPurposeEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let input: &[u8] = b"SGVsbG8gV29ybGQ="; // "Hello World" in base64
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 12 };
    let mut output = vec![0u8; 16]; // Allocate enough space for the output
    let decode_table: [u8; 256] = {
        let mut table = [0u8; 256];
        for (i, &byte) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
            table[byte as usize] = i as u8;
        }
        table[b'=' as usize] = 255; // Set '=' to an invalid value
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    match decode_helper(input, &estimate, &mut output, &decode_table, decode_allow_trailing_bits, padding_mode) {
        Ok(metadata) => {
            assert_eq!(metadata.decoded_len, 12);
            assert_eq!(output[..11], b"Hello World\0"); // Should match the decoded output
            assert!(metadata.padding_offset.is_some());
        }
        Err(_) => panic!("Should not have returned an error"),
    }
}

#[test]
#[should_panic]
fn test_decode_helper_invalid_chunk_processing() {
    struct GeneralPurposeEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let input: &[u8] = b"SGVsbG8gV29ybGQ"; // Missing padding
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 12 };
    let mut output = vec![0u8; 16];
    let decode_table: [u8; 256] = {
        let mut table = [0u8; 256];
        for (i, &byte) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
            table[byte as usize] = i as u8;
        }
        table[b'=' as usize] = 255; // Set '=' to an invalid value
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    match decode_helper(input, &estimate, &mut output, &decode_table, decode_allow_trailing_bits, padding_mode) {
        Ok(_) => panic!("Expecting an error due to invalid chunk processing"),
        Err(_) => {}
    }
}

