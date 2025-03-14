// Answer 0

#[test]
fn test_complete_quads_len_valid_case() {
    const DECODE_TABLE: [u8; 256] = [
        // Assume this array has been initialized correctly with valid base64 decode values
        // The actual values should be filled out according to the base64 specification
        // For illustration, here are just some values:
        255, 255, 255, 255, 255, 255, 255, 255, // 0-7
        // ... fill in for all 256 ASCII values
        // Example valid entry for 'A' (index 65)
        0, // A
        // Example valid entry for 'B' (index 66)
        1, // B
        // The rest should be populated similarly
    ];

    let input = b"QUJD"; // 'A', 'B', 'C' in Base64
    let input_len_rem = 0;
    let output_len = 3; // Output buffer should fit the 3 bytes of data
    let result = complete_quads_len(input, input_len_rem, output_len, &DECODE_TABLE);
    assert_eq!(result, Ok(4)); // 4 complete quads, therefore returns 4
}

#[test]
fn test_complete_quads_len_output_too_small() {
    const DECODE_TABLE: [u8; 256] = [255; 256]; // All invalid for testing

    let input = b"QUJD"; // Valid base64 input
    let input_len_rem = 0;
    let output_len = 2; // Not enough space for the output
    let result = complete_quads_len(input, input_len_rem, output_len, &DECODE_TABLE);
    assert_eq!(result, Err(DecodeSliceError::OutputSliceTooSmall));
}

#[test]
fn test_complete_quads_len_invalid_byte() {
    const DECODE_TABLE: [u8; 256] = [
        255, 255, 255, 255, 255, 255, 255, 255, // 0-7
        // ... fill in for all 256 ASCII values
        255, // Lets say we made `Z` invalid for instance
        // Example invalid entry for 'Y' (index 89)
        255, // Y is now invalid
        // ...
    ];

    let input = b"QUJY"; // Last byte is invalid 'Y' which is not in our decode table
    let input_len_rem = 1;
    let output_len = 3;
    let result = complete_quads_len(input, input_len_rem, output_len, &DECODE_TABLE);
    assert_eq!(result, Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(3, 89))));
}

