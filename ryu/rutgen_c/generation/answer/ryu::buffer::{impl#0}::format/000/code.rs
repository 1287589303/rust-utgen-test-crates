// Answer 0

#[test]
fn test_format_nan() {
    struct FloatNaN;

    impl Copy for FloatNaN {}
    
    impl Sealed for FloatNaN {
        fn is_nonfinite(self) -> bool {
            true
        }
        fn format_nonfinite(self) -> &'static str {
            NAN
        }
        unsafe fn write_to_ryu_buffer(self, _result: *mut u8) -> usize {
            0 // No buffer write since it's NaN
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format(FloatNaN);
    assert_eq!(result, NAN);
}

#[test]
fn test_format_positive_infinity() {
    struct FloatPositiveInfinity;

    impl Copy for FloatPositiveInfinity {}

    impl Sealed for FloatPositiveInfinity {
        fn is_nonfinite(self) -> bool {
            true
        }
        fn format_nonfinite(self) -> &'static str {
            INFINITY
        }
        unsafe fn write_to_ryu_buffer(self, _result: *mut u8) -> usize {
            0 // No buffer write since it's infinity
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format(FloatPositiveInfinity);
    assert_eq!(result, INFINITY);
}

#[test]
fn test_format_negative_infinity() {
    struct FloatNegativeInfinity;

    impl Copy for FloatNegativeInfinity {}

    impl Sealed for FloatNegativeInfinity {
        fn is_nonfinite(self) -> bool {
            true
        }
        fn format_nonfinite(self) -> &'static str {
            NEG_INFINITY
        }
        unsafe fn write_to_ryu_buffer(self, _result: *mut u8) -> usize {
            0 // No buffer write since it's -infinity
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format(FloatNegativeInfinity);
    assert_eq!(result, NEG_INFINITY);
}

#[test]
fn test_format_finite_value() {
    struct FloatFinite(f64);

    impl Copy for FloatFinite(f64) {}

    impl Sealed for FloatFinite {
        fn is_nonfinite(self) -> bool {
            false
        }
        fn format_nonfinite(self) -> &'static str {
            unreachable!() // This should never be called for finite values
        }
        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let bytes = format!("{}", self.0).as_bytes();
            (0..bytes.len()).for_each(|i| *result.add(i) = bytes[i]);
            bytes.len()
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format(FloatFinite(3.14));
    assert_eq!(result, "3.14");
}

