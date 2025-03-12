// Answer 0

#[test]
fn test_format_nan() {
    struct NaNFloat;

    impl Copy for NaNFloat {}

    impl Sealed for NaNFloat {
        fn is_nonfinite(self) -> bool {
            true
        }

        fn format_nonfinite(self) -> &'static str {
            NAN
        }

        unsafe fn write_to_ryu_buffer(self, _: *mut u8) -> usize {
            0 // no actual data to write for NaN
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format(NaNFloat);
}

#[test]
fn test_format_infinity() {
    struct InfinityFloat;

    impl Copy for InfinityFloat {}

    impl Sealed for InfinityFloat {
        fn is_nonfinite(self) -> bool {
            true
        }

        fn format_nonfinite(self) -> &'static str {
            INFINITY
        }

        unsafe fn write_to_ryu_buffer(self, _: *mut u8) -> usize {
            0 // no actual data to write for positive infinity
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format(InfinityFloat);
}

#[test]
fn test_format_negative_infinity() {
    struct NegativeInfinityFloat;

    impl Copy for NegativeInfinityFloat {}

    impl Sealed for NegativeInfinityFloat {
        fn is_nonfinite(self) -> bool {
            true
        }

        fn format_nonfinite(self) -> &'static str {
            NEG_INFINITY
        }

        unsafe fn write_to_ryu_buffer(self, _: *mut u8) -> usize {
            0 // no actual data to write for negative infinity
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format(NegativeInfinityFloat);
}

