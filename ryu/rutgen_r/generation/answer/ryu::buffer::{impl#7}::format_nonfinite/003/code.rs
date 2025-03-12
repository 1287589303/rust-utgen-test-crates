// Answer 0

#[test]
fn test_format_nonfinite_infinity() {
    struct TestFloat {
        value: f64,
    }

    impl TestFloat {
        fn to_bits(self) -> u64 {
            self.value.to_bits()
        }

        fn format_nonfinite(self) -> &'static str {
            const MANTISSA_MASK: u64 = 0x000fffffffffffff;
            const SIGN_MASK: u64 = 0x8000000000000000;
            let bits = self.to_bits();
            if bits & MANTISSA_MASK != 0 {
                "NAN"
            } else if bits & SIGN_MASK != 0 {
                "NEG_INFINITY"
            } else {
                "INFINITY"
            }
        }
    }

    let test_value = TestFloat { value: f64::INFINITY };
    assert_eq!(test_value.format_nonfinite(), "INFINITY");
}

#[test]
fn test_format_nonfinite_zero() {
    struct TestFloat {
        value: f64,
    }

    impl TestFloat {
        fn to_bits(self) -> u64 {
            self.value.to_bits()
        }

        fn format_nonfinite(self) -> &'static str {
            const MANTISSA_MASK: u64 = 0x000fffffffffffff;
            const SIGN_MASK: u64 = 0x8000000000000000;
            let bits = self.to_bits();
            if bits & MANTISSA_MASK != 0 {
                "NAN"
            } else if bits & SIGN_MASK != 0 {
                "NEG_INFINITY"
            } else {
                "INFINITY"
            }
        }
    }

    let test_value = TestFloat { value: 0.0 };
    assert_eq!(test_value.format_nonfinite(), "INFINITY");
}

#[test]
fn test_format_nonfinite_large_positive_number() {
    struct TestFloat {
        value: f64,
    }

    impl TestFloat {
        fn to_bits(self) -> u64 {
            self.value.to_bits()
        }

        fn format_nonfinite(self) -> &'static str {
            const MANTISSA_MASK: u64 = 0x000fffffffffffff;
            const SIGN_MASK: u64 = 0x8000000000000000;
            let bits = self.to_bits();
            if bits & MANTISSA_MASK != 0 {
                "NAN"
            } else if bits & SIGN_MASK != 0 {
                "NEG_INFINITY"
            } else {
                "INFINITY"
            }
        }
    }

    let test_value = TestFloat { value: 1e300 };
    assert_eq!(test_value.format_nonfinite(), "INFINITY");
}

