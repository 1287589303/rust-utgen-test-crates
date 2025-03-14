// Answer 0

#[test]
fn test_add_padding_no_padding() {
    let unpadded_output_len = 4;
    let mut output = [0u8; 4]; // Create a slice of length at least 2
    let result = add_padding(unpadded_output_len, &mut output);
    assert_eq!(result, 0);
    assert_eq!(output, [0, 0, 0, 0]);
}

#[test]
fn test_add_padding_one_byte_padding() {
    let unpadded_output_len = 5;
    let mut output = [0u8; 4]; // Create a slice of length at least 2
    let result = add_padding(unpadded_output_len, &mut output);
    assert_eq!(result, 3); // Should return 3 padding bytes
    assert_eq!(output, [PAD_BYTE, PAD_BYTE, PAD_BYTE, 0]);
}

#[test]
fn test_add_padding_two_bytes_padding() {
    let unpadded_output_len = 6;
    let mut output = [0u8; 4]; // Create a slice of length at least 2
    let result = add_padding(unpadded_output_len, &mut output);
    assert_eq!(result, 2); // Should return 2 padding bytes
    assert_eq!(output, [PAD_BYTE, PAD_BYTE, 0, 0]);
}

#[test]
fn test_add_padding_three_bytes_padding() {
    let unpadded_output_len = 7;
    let mut output = [0u8; 4]; // Create a slice of length at least 2
    let result = add_padding(unpadded_output_len, &mut output);
    assert_eq!(result, 1); // Should return 1 padding byte
    assert_eq!(output, [PAD_BYTE, 0, 0, 0]);
}

