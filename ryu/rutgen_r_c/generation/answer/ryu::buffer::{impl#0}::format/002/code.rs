// Answer 0

#[test]
fn test_format_finite_float() {
    struct FiniteFloat;
    impl Copy for FiniteFloat {}
    
    impl Sealed for FiniteFloat {
        fn is_nonfinite(self) -> bool {
            false
        }
        fn format_nonfinite(self) -> &'static str {
            ""
        }
        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let bytes = b"3.14";
            (0..bytes.len()).for_each(|i| *result.add(i) = bytes[i]);
            bytes.len()
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format(FiniteFloat);
    assert_eq!(result, "3.14");
}

#[test]
fn test_format_finite_float_zero() {
    struct ZeroFloat;
    impl Copy for ZeroFloat {}
    
    impl Sealed for ZeroFloat {
        fn is_nonfinite(self) -> bool {
            false
        }
        fn format_nonfinite(self) -> &'static str {
            ""
        }
        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let bytes = b"0.0";
            (0..bytes.len()).for_each(|i| *result.add(i) = bytes[i]);
            bytes.len()
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format(ZeroFloat);
    assert_eq!(result, "0.0");
}

#[test]
fn test_format_finite_float_positive() {
    struct PositiveFloat;
    impl Copy for PositiveFloat {}
    
    impl Sealed for PositiveFloat {
        fn is_nonfinite(self) -> bool {
            false
        }
        fn format_nonfinite(self) -> &'static str {
            ""
        }
        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let bytes = b"1.618";
            (0..bytes.len()).for_each(|i| *result.add(i) = bytes[i]);
            bytes.len()
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format(PositiveFloat);
    assert_eq!(result, "1.618");
}

