// Answer 0

#[test]
fn test_format_finite_large_positive_float() {
    struct FloatValue;

    impl Copy for FloatValue {}

    impl Sealed for FloatValue {
        fn is_nonfinite(self) -> bool {
            false
        }

        fn format_nonfinite(self) -> &'static str {
            ""
        }

        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let value: f64 = 1e38; // large positive float
            let formatted = format!("{}", value);
            let bytes = formatted.as_bytes();
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), result, bytes.len());
            bytes.len()
        }
    }

    let mut buffer = Buffer::new();
    let _ = buffer.format_finite(FloatValue);
}

#[test]
fn test_format_finite_large_negative_float() {
    struct FloatValue;

    impl Copy for FloatValue {}

    impl Sealed for FloatValue {
        fn is_nonfinite(self) -> bool {
            false
        }

        fn format_nonfinite(self) -> &'static str {
            ""
        }

        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let value: f64 = -1e38; // large negative float
            let formatted = format!("{}", value);
            let bytes = formatted.as_bytes();
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), result, bytes.len());
            bytes.len()
        }
    }

    let mut buffer = Buffer::new();
    let _ = buffer.format_finite(FloatValue);
}

#[test]
fn test_format_finite_nan() {
    struct FloatValue;

    impl Copy for FloatValue {}

    impl Sealed for FloatValue {
        fn is_nonfinite(self) -> bool {
            true
        }

        fn format_nonfinite(self) -> &'static str {
            "NaN"
        }

        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let value: f64 = f64::NAN; // NaN
            let formatted = format!("{}", value);
            let bytes = formatted.as_bytes();
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), result, bytes.len());
            bytes.len()
        }
    }

    let mut buffer = Buffer::new();
    let _ = buffer.format_finite(FloatValue);
}

#[test]
fn test_format_finite_infinity() {
    struct FloatValue;

    impl Copy for FloatValue {}

    impl Sealed for FloatValue {
        fn is_nonfinite(self) -> bool {
            true
        }

        fn format_nonfinite(self) -> &'static str {
            "inf"
        }

        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let value: f64 = f64::INFINITY; // Infinity
            let formatted = format!("{}", value);
            let bytes = formatted.as_bytes();
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), result, bytes.len());
            bytes.len()
        }
    }

    let mut buffer = Buffer::new();
    let _ = buffer.format_finite(FloatValue);
}

#[test]
fn test_format_finite_negative_infinity() {
    struct FloatValue;

    impl Copy for FloatValue {}

    impl Sealed for FloatValue {
        fn is_nonfinite(self) -> bool {
            true
        }

        fn format_nonfinite(self) -> &'static str {
            "-inf"
        }

        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let value: f64 = f64::NEG_INFINITY; // Negative Infinity
            let formatted = format!("{}", value);
            let bytes = formatted.as_bytes();
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), result, bytes.len());
            bytes.len()
        }
    }

    let mut buffer = Buffer::new();
    let _ = buffer.format_finite(FloatValue);
}

