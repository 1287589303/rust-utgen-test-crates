// Answer 0

#[test]
fn test_write_mantissa_output_below_all_thresholds() {
    use std::ptr;

    const DIGIT_TABLE: [u8; 200] = *b"000102030405060708090a0b0c0d0e0f101112131415161718192021222324252627282930313233343536373839404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecf";

    let mut output: u32 = 5; // below 10,000, 100, and 10
    let mut buffer: [u8; 10] = [0; 10];
    let result_ptr = buffer.as_mut_ptr().offset(10); // Point to the end of the buffer

    unsafe {
        write_mantissa(output, result_ptr);
    }

    // Check results in buffer, should only be '05' at the end since output was 5
    assert_eq!(&buffer[8..10], b"05");
    assert_eq!(&buffer[0..8], &[0; 8]); // Ensure no other values were written
}

#[test]
fn test_write_mantissa_output_exactly_at_boundary_99() {
    use std::ptr;

    const DIGIT_TABLE: [u8; 200] = *b"000102030405060708090a0b0c0d0e0f101112131415161718192021222324252627282930313233343536373839404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecf";

    let mut output: u32 = 99; // below 10,000 and 100, but at boundary of 10
    let mut buffer: [u8; 10] = [0; 10];
    let result_ptr = buffer.as_mut_ptr().offset(10); // Point to the end of the buffer

    unsafe {
        write_mantissa(output, result_ptr);
    }

    // Check results in buffer, should only be '99' at the end since output was 99
    assert_eq!(&buffer[8..10], b"99");
    assert_eq!(&buffer[0..8], &[0; 8]); // Ensure no other values were written
}

#[test]
fn test_write_mantissa_output_exactly_10() {
    use std::ptr;

    const DIGIT_TABLE: [u8; 200] = *b"000102030405060708090a0b0c0d0e0f101112131415161718192021222324252627282930313233343536373839404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecf";

    let mut output: u32 = 10; // below 10,000 and 100, but at boundary of 10
    let mut buffer: [u8; 10] = [0; 10];
    let result_ptr = buffer.as_mut_ptr().offset(10); // Point to the end of the buffer

    unsafe {
        write_mantissa(output, result_ptr);
    }

    // Check results in buffer, should only be '10' at the end since output was 10
    assert_eq!(&buffer[8..10], b"10");
    assert_eq!(&buffer[0..8], &[0; 8]); // Ensure no other values were written
}

