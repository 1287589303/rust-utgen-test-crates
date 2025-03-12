// Answer 0

#[test]
fn test_format_nonfinite_nan() {
    struct NonFinite {
        value: f64,
    }

    impl NonFinite {
        fn to_bits(&self) -> u64 {
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

    let nan_value = NonFinite { value: f64::NAN };
    assert_eq!(nan_value.format_nonfinite(), "NAN");
}

