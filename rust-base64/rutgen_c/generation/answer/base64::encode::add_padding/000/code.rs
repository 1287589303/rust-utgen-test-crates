// Answer 0

#[test]
fn test_add_padding_no_padding_needed() {
    let mut output = [0u8; 4];
    let unpadded_output_len = 8; // Length is multiple of 4, so no padding needed
    let padding_written = add_padding(unpadded_output_len, &mut output);
    assert_eq!(padding_written, 0);
    assert_eq!(&output[..], &[0, 0, 0, 0]);
}

#[test]
fn test_add_padding_one_byte_padding() {
    let mut output = [0u8; 4];
    let unpadded_output_len = 6; // Needs 1 byte of padding
    let padding_written = add_padding(unpadded_output_len, &mut output);
    assert_eq!(padding_written, 1);
    assert_eq!(&output[..1], &[PAD_BYTE]);
    assert_eq!(&output[1..], &[0, 0, 0]);
}

#[test]
fn test_add_padding_two_bytes_padding() {
    let mut output = [0u8; 4];
    let unpadded_output_len = 5; // Needs 2 bytes of padding
    let padding_written = add_padding(unpadded_output_len, &mut output);
    assert_eq!(padding_written, 2);
    assert_eq!(&output[..2], &[PAD_BYTE, PAD_BYTE]);
    assert_eq!(&output[2..], &[0, 0]);
}

#[test]
fn test_add_padding_three_bytes_padding() {
    let mut output = [0u8; 4];
    let unpadded_output_len = 4; // Needs 3 bytes of padding
    let padding_written = add_padding(unpadded_output_len, &mut output);
    assert_eq!(padding_written, 3);
    assert_eq!(&output[..3], &[PAD_BYTE, PAD_BYTE, PAD_BYTE]);
    assert_eq!(&output[3..], &[0]);
}

#[test]
fn test_add_padding_full_capacity() {
    let mut output = [0u8; 4];
    let unpadded_output_len = 2; // Needs 2 bytes of padding
    let padding_written = add_padding(unpadded_output_len, &mut output);
    assert_eq!(padding_written, 2);
    assert_eq!(&output[..2], &[PAD_BYTE, PAD_BYTE]);
    assert_eq!(&output[2..], &[0, 0]);
}

