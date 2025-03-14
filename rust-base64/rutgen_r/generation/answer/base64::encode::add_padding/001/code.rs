// Answer 0

#[test]
fn test_add_padding_no_padding_needed() {
    let mut output = [0u8; 2];
    let unpadded_output_len = 8; // already a multiple of 4
    let pad_bytes = add_padding(unpadded_output_len, &mut output);
    assert_eq!(pad_bytes, 0);
    assert_eq!(output, [0, 0]); // no padding should be written
}

#[test]
fn test_add_padding_one_byte_padding() {
    let mut output = [0u8; 2];
    let unpadded_output_len = 9; // requires 1 byte of padding
    let pad_bytes = add_padding(unpadded_output_len, &mut output);
    assert_eq!(pad_bytes, 1);
    assert_eq!(output[0], PAD_BYTE); // first byte should be padding
    assert_eq!(output[1], 0); // second byte should remain unchanged
}

#[test]
fn test_add_padding_two_bytes_padding() {
    let mut output = [0u8; 2];
    let unpadded_output_len = 10; // requires 2 bytes of padding
    let pad_bytes = add_padding(unpadded_output_len, &mut output);
    assert_eq!(pad_bytes, 2);
    assert_eq!(output[0], PAD_BYTE); // first byte should be padding
    assert_eq!(output[1], PAD_BYTE); // second byte should also be padding
}

