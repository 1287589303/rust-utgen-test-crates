// Answer 0

#[test]
fn test_decode_suffix_valid_case_with_padding() {
    struct DecodeMetadata {
        output_index: usize,
        padding_index: Option<usize>,
    }

    struct DecodeSliceError;

    const PAD_BYTE: u8 = b'='; 
    const INVALID_VALUE: u8 = 0xFF; 
    const DECODE_TABLE: &[u8; 256] = &[INVALID_VALUE; 256];

    let input = b"Zm9v"; // base64 for "foo"
    let input_index = 0;
    let mut output = [0u8; 4];
    let mut output_index = 0;
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        DECODE_TABLE,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.output_index, 3); // Expecting "foo" to be decoded
    assert_eq!(metadata.padding_index, None);
}

#[test]
fn test_decode_suffix_invalid_case_with_padding() {
    struct DecodeMetadata {
        output_index: usize,
        padding_index: Option<usize>,
    }

    struct DecodeSliceError;

    const PAD_BYTE: u8 = b'='; 
    const INVALID_VALUE: u8 = 0xFF; 
    const DECODE_TABLE: &[u8; 256] = &[INVALID_VALUE; 256];

    let input = b"Zm8="; // base64 for "mo" with valid padding
    let input_index = 0;
    let mut output = [0u8; 4];
    let mut output_index = 0;
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    let result = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        DECODE_TABLE,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.output_index, 2); // Expecting "mo" to be decoded
    assert_eq!(metadata.padding_index, Some(3)); // Padding at last index
}

