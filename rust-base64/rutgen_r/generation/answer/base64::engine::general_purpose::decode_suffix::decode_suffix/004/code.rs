// Answer 0

#[test]
fn test_decode_suffix_with_two_padding_bytes() {
    struct DecodeMetadata {
        output_index: usize,
        padding_index: Option<usize>,
    }

    struct DecodeSliceError;

    const PAD_BYTE: u8 = b'='; // Assuming '=' is the padding byte.
    const INVALID_VALUE: u8 = 255; // Value indicating an invalid byte.
    const DECODE_TABLE: [u8; 256] = [INVALID_VALUE; 256]; // Initialize with invalid values.
    
    // Setting up a valid decode table for testing.
    DECODE_TABLE[b'A' as usize] = 0;
    DECODE_TABLE[b'B' as usize] = 1;
    DECODE_TABLE[b'C' as usize] = 2;
    DECODE_TABLE[b'D' as usize] = 3;

    // Input emulating the case where we have two bytes followed by `==`.
    let input = b"ABCD==";
    let input_index = 4;
    let mut output = [0u8; 6];
    let mut output_index = 0;
    
    #[derive(Debug)]
    enum DecodePaddingMode {
        Indifferent,
        RequireCanonical,
        RequireNone,
    }

    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &DECODE_TABLE,
        true,
        padding_mode,
    );

    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.output_index, 6);
    assert_eq!(metadata.padding_index, Some(4));
}

#[test]
fn test_decode_suffix_with_one_padding_byte() {
    struct DecodeMetadata {
        output_index: usize,
        padding_index: Option<usize>,
    }

    struct DecodeSliceError;

    const PAD_BYTE: u8 = b'='; // Assuming '=' is the padding byte.
    const INVALID_VALUE: u8 = 255; // Value indicating an invalid byte.
    const DECODE_TABLE: [u8; 256] = [INVALID_VALUE; 256]; // Initialize with invalid values.
    
    // Setting up a valid decode table for testing.
    DECODE_TABLE[b'A' as usize] = 0;
    DECODE_TABLE[b'B' as usize] = 1;
    DECODE_TABLE[b'C' as usize] = 2;
    
    // Input emulating the case where we have one byte followed by `=`.
    let input = b"ABCD=";
    let input_index = 4;
    let mut output = [0u8; 6];
    let mut output_index = 0;
    
    #[derive(Debug)]
    enum DecodePaddingMode {
        Indifferent,
        RequireCanonical,
        RequireNone,
    }

    let padding_mode = DecodePaddingMode::RequireCanonical;

    let result = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &DECODE_TABLE,
        true,
        padding_mode,
    );

    assert!(result.is_ok());
    let metadata = result.unwrap();
    assert_eq!(metadata.output_index, 5);
    assert_eq!(metadata.padding_index, Some(4));
}

