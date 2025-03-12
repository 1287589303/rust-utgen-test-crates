// Answer 0

#[test]
fn test_format_finite_float() {
    struct FloatTest(u32);
    
    unsafe impl Sealed for FloatTest {
        fn is_nonfinite(self) -> bool {
            false
        }

        fn format_nonfinite(self) -> &'static str {
            "finite"
        }

        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let value = self.0.to_string();
            let bytes = value.as_bytes();
            let len = bytes.len();
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), result, len);
            len
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format_finite(FloatTest(42));
    assert_eq!(result, "42");
}

#[test]
fn test_format_finite_float_with_large_value() {
    struct FloatTest(u32);
    
    unsafe impl Sealed for FloatTest {
        fn is_nonfinite(self) -> bool {
            false
        }

        fn format_nonfinite(self) -> &'static str {
            "finite"
        }

        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let value = self.0.to_string();
            let bytes = value.as_bytes();
            let len = bytes.len();
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), result, len);
            len
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format_finite(FloatTest(99999999));
    assert_eq!(result, "99999999");
}

#[test]
fn test_format_finite_edge_case() {
    struct FloatTest(u64);
    
    unsafe impl Sealed for FloatTest {
        fn is_nonfinite(self) -> bool {
            false
        }

        fn format_nonfinite(self) -> &'static str {
            "finite"
        }

        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let value = self.0.to_string();
            let bytes = value.as_bytes();
            let len = bytes.len();
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), result, len);
            len
        }
    }

    let mut buffer = Buffer::new();
    let large_value = FloatTest(12345678901234567890);
    let result = buffer.format_finite(large_value);
    assert_eq!(result, "12345678901234567890");
}

