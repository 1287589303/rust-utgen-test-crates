// Answer 0

#[test]
fn test_format_finite_float() {
    struct FloatMock;

    impl Float for FloatMock {
        fn write_to_ryu_buffer(&self, buffer: *mut u8) -> usize {
            // Mock implementation that writes a fixed string representation of a float to buffer
            let data = b"1.23"; // The representation of the float
            unsafe {
                std::ptr::copy_nonoverlapping(data.as_ptr(), buffer, data.len());
                data.len()
            }
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format_finite(FloatMock);
    assert_eq!(result, "1.23");
}

#[test]
fn test_format_finite_large_float() {
    struct FloatMockLarge;

    impl Float for FloatMockLarge {
        fn write_to_ryu_buffer(&self, buffer: *mut u8) -> usize {
            let data = b"1234567890.1234567890"; // Larger float representation
            unsafe {
                std::ptr::copy_nonoverlapping(data.as_ptr(), buffer, data.len());
                data.len()
            }
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format_finite(FloatMockLarge);
    assert_eq!(result, "1234567890.1234567890");
}

#[test]
fn test_format_finite_zero() {
    struct FloatMockZero;

    impl Float for FloatMockZero {
        fn write_to_ryu_buffer(&self, buffer: *mut u8) -> usize {
            let data = b"0.0"; // Representation of zero
            unsafe {
                std::ptr::copy_nonoverlapping(data.as_ptr(), buffer, data.len());
                data.len()
            }
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format_finite(FloatMockZero);
    assert_eq!(result, "0.0");
}

#[test]
fn test_format_finite_negative_float() {
    struct FloatMockNegative;

    impl Float for FloatMockNegative {
        fn write_to_ryu_buffer(&self, buffer: *mut u8) -> usize {
            let data = b"-2.5"; // Representation of a negative float
            unsafe {
                std::ptr::copy_nonoverlapping(data.as_ptr(), buffer, data.len());
                data.len()
            }
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format_finite(FloatMockNegative);
    assert_eq!(result, "-2.5");
}

