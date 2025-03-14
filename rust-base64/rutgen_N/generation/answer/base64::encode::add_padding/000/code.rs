// Answer 0

#[test]
fn test_add_padding_no_padding_needed() {
    let mut output = [0u8; 4];
    let result = add_padding(4, &mut output);
    assert_eq!(result, 0);
    assert_eq!(&output[0..4], &[0, 0, 0, 0]);
}

#[test]
fn test_add_padding_one_padding_byte() {
    let mut output = [0u8; 4];
    let result = add_padding(3, &mut output);
    assert_eq!(result, 1);
    assert_eq!(&output[0..1], &[PAD_BYTE]);
    assert_eq!(&output[1..4], &[0, 0, 0]);
}

#[test]
fn test_add_padding_two_padding_bytes() {
    let mut output = [0u8; 4];
    let result = add_padding(2, &mut output);
    assert_eq!(result, 2);
    assert_eq!(&output[0..2], &[PAD_BYTE, PAD_BYTE]);
    assert_eq!(&output[2..4], &[0, 0]);
}

#[test]
fn test_add_padding_three_padding_bytes() {
    let mut output = [0u8; 4];
    let result = add_padding(1, &mut output);
    assert_eq!(result, 3);
    assert_eq!(&output[0..3], &[PAD_BYTE, PAD_BYTE, PAD_BYTE]);
    assert_eq!(output[3], 0);
}

#[test]
fn test_add_padding_multiple_of_four() {
    let mut output = [0u8; 6];
    let result = add_padding(8, &mut output);
    assert_eq!(result, 0);
    assert_eq!(&output[0..6], &[0, 0, 0, 0, 0, 0]);
}

