// Answer 0

#[test]
fn test_decode_helper_valid_input_length_multiple_of_4() {
    let input: &[u8] = b"YW55IGNhbmEgZW5jb2RlZA=="; // valid base64 encoded string
    let mut output = vec![0u8; (input.len() * 3) / 4];
    let estimate = GeneralPurposeEstimate { rem: 0, conservative_decoded_len: output.len() };
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // fill with valid decode values
        for (i, c) in "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".bytes().enumerate() {
            table[c as usize] = i as u8;
        }
        table[b'=' as usize] = 0; // Valid padding
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let _ = decode_helper(input, &estimate, &mut output, &decode_table, decode_allow_trailing_bits, padding_mode);
}

#[test]
fn test_decode_helper_valid_input_with_rem_1() {
    let input: &[u8] = b"YW55IGNhbmEgZW5jb2xl"; // valid base64 with remainder
    let mut output = vec![0u8; (input.len() * 3) / 4];
    let estimate = GeneralPurposeEstimate { rem: 1, conservative_decoded_len: output.len() };
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for (i, c) in "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".bytes().enumerate() {
            table[c as usize] = i as u8;
        }
        table[b'=' as usize] = 0; // Valid padding
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let _ = decode_helper(input, &estimate, &mut output, &decode_table, decode_allow_trailing_bits, padding_mode);
}

#[test]
fn test_decode_helper_valid_input_with_rem_2() {
    let input: &[u8] = b"YW55IGNhbmEgZW5jb2xlYw=="; // valid base64 with remainder
    let mut output = vec![0u8; (input.len() * 3) / 4];
    let estimate = GeneralPurposeEstimate { rem: 2, conservative_decoded_len: output.len() };
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for (i, c) in "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".bytes().enumerate() {
            table[c as usize] = i as u8;
        }
        table[b'=' as usize] = 0; // Valid padding
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let _ = decode_helper(input, &estimate, &mut output, &decode_table, decode_allow_trailing_bits, padding_mode);
}

#[test]
fn test_decode_helper_valid_input_with_rem_3() {
    let input: &[u8] = b"YW55IGNhbmEgZW5jb2xlYXRh"; // valid base64 with remainder
    let mut output = vec![0u8; (input.len() * 3) / 4];
    let estimate = GeneralPurposeEstimate { rem: 3, conservative_decoded_len: output.len() };
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for (i, c) in "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".bytes().enumerate() {
            table[c as usize] = i as u8;
        }
        table[b'=' as usize] = 0; // Valid padding
        table
    };
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireCanonical;

    let _ = decode_helper(input, &estimate, &mut output, &decode_table, decode_allow_trailing_bits, padding_mode);
}

