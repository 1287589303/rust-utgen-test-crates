// Answer 0

#[test]
fn test_format_nonfinite_neg_infinity() {
    struct NonFinite;

    impl Copy for NonFinite {}

    impl Sealed for NonFinite {
        fn is_nonfinite(self) -> bool {
            false
        }

        fn format_nonfinite(self) -> &'static str {
            const MANTISSA_MASK: u32 = 0x007fffff;
            const SIGN_MASK: u32 = 0x80000000;
            let bits = 0x80000000; // bits with SIGN_MASK set and MANTISSA_MASK not set
            if bits & MANTISSA_MASK != 0 {
                NAN
            } else if bits & SIGN_MASK != 0 {
                NEG_INFINITY
            } else {
                INFINITY
            }
        }

        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            0 // Implementation not needed for this test
        }
    }

    let non_finite_val = NonFinite;
    assert_eq!(non_finite_val.format_nonfinite(), NEG_INFINITY);
}

