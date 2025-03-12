// Answer 0

#[test]
fn test_format_finite_f32() {
    struct F32Wrapper(f32);
    unsafe impl raw::Float for F32Wrapper {
        fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            // Example dummy implementation just for the sake of testing
            let value = format!("{}", self.0);
            let bytes = value.as_bytes();
            let len = bytes.len();
            ptr::copy_nonoverlapping(bytes.as_ptr(), result, len);
            len
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format_finite(F32Wrapper(3.14));
    assert_eq!(result, "3.14");
}

#[test]
fn test_format_finite_f64() {
    struct F64Wrapper(f64);
    unsafe impl raw::Float for F64Wrapper {
        fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            // Example dummy implementation just for the sake of testing
            let value = format!("{}", self.0);
            let bytes = value.as_bytes();
            let len = bytes.len();
            ptr::copy_nonoverlapping(bytes.as_ptr(), result, len);
            len
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format_finite(F64Wrapper(2.718));
    assert_eq!(result, "2.718");
}

#[test]
fn test_format_finite_zero() {
    struct F32Wrapper(f32);
    unsafe impl raw::Float for F32Wrapper {
        fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let value = format!("{}", self.0);
            let bytes = value.as_bytes();
            let len = bytes.len();
            ptr::copy_nonoverlapping(bytes.as_ptr(), result, len);
            len
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format_finite(F32Wrapper(0.0));
    assert_eq!(result, "0");
}

#[test]
fn test_format_finite_negative() {
    struct F32Wrapper(f32);
    unsafe impl raw::Float for F32Wrapper {
        fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let value = format!("{}", self.0);
            let bytes = value.as_bytes();
            let len = bytes.len();
            ptr::copy_nonoverlapping(bytes.as_ptr(), result, len);
            len
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format_finite(F32Wrapper(-4.2));
    assert_eq!(result, "-4.2");
}

#[test]
fn test_format_finite_large_number() {
    struct F64Wrapper(f64);
    unsafe impl raw::Float for F64Wrapper {
        fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let value = format!("{}", self.0);
            let bytes = value.as_bytes();
            let len = bytes.len();
            ptr::copy_nonoverlapping(bytes.as_ptr(), result, len);
            len
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format_finite(F64Wrapper(1e30));
    assert_eq!(result, "1000000000000000000000000000000");
}

