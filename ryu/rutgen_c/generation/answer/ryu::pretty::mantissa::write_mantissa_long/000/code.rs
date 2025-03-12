// Answer 0

#[test]
fn test_write_mantissa_long_with_large_output() {
    const DIGIT_COUNT: usize = 20;
    let mut buffer = [0u8; DIGIT_COUNT];
    let output: u64 = 12345678901234567890;
    let result_ptr = buffer.as_mut_ptr().add(DIGIT_COUNT);

    unsafe {
        write_mantissa_long(output, result_ptr);
    }

    assert_eq!(&buffer[DIGIT_COUNT - 10..], b"1234567890");
}

#[test]
fn test_write_mantissa_long_with_small_output() {
    const DIGIT_COUNT: usize = 20;
    let mut buffer = [0u8; DIGIT_COUNT];
    let output: u64 = 123;
    let result_ptr = buffer.as_mut_ptr().add(DIGIT_COUNT);

    unsafe {
        write_mantissa_long(output, result_ptr);
    }

    assert_eq!(&buffer[DIGIT_COUNT - 2..], b"12");
}

#[test]
fn test_write_mantissa_long_edge_case() {
    const DIGIT_COUNT: usize = 20;
    let mut buffer = [0u8; DIGIT_COUNT];
    let output: u64 = 10000000000; // an exact power of ten
    let result_ptr = buffer.as_mut_ptr().add(DIGIT_COUNT);

    unsafe {
        write_mantissa_long(output, result_ptr);
    }

    assert_eq!(&buffer[DIGIT_COUNT - 10..], b"1000000000");
}

