// Answer 0

#[test]
fn test_add_padding_no_padding_needed() {
    let unpadded_output_len = 0; // Multiple of 4
    let mut output = [0u8; 4];
    let pad_bytes = add_padding(unpadded_output_len, &mut output);
    assert_eq!(pad_bytes, 0);
    assert_eq!(output, [0, 0, 0, 0]);
}

#[test]
fn test_add_padding_three_bytes_padding_needed() {
    let unpadded_output_len = 1; // 1 % 4 = 1, so 4 - 1 = 3
    let mut output = [0u8; 4];
    let pad_bytes = add_padding(unpadded_output_len, &mut output);
    assert_eq!(pad_bytes, 3);
    assert_eq!(output[0..3], [PAD_BYTE; 3]);
}

#[test]
fn test_add_padding_two_bytes_padding_needed() {
    let unpadded_output_len = 2; // 2 % 4 = 2, so 4 - 2 = 2
    let mut output = [0u8; 4];
    let pad_bytes = add_padding(unpadded_output_len, &mut output);
    assert_eq!(pad_bytes, 2);
    assert_eq!(output[0..2], [PAD_BYTE; 2]);
}

#[test]
fn test_add_padding_one_byte_padding_needed() {
    let unpadded_output_len = 3; // 3 % 4 = 3, so 4 - 3 = 1
    let mut output = [0u8; 4];
    let pad_bytes = add_padding(unpadded_output_len, &mut output);
    assert_eq!(pad_bytes, 1);
    assert_eq!(output[0], PAD_BYTE);
}

#[test]
fn test_add_padding_four_bytes_padding_needed() {
    let unpadded_output_len = 4; // 4 % 4 = 0, so padding needed = 0
    let mut output = [0u8; 4];
    let pad_bytes = add_padding(unpadded_output_len, &mut output);
    assert_eq!(pad_bytes, 0);
    assert_eq!(output, [0, 0, 0, 0]);
}

