// Answer 0

#[test]
fn test_format_nonfinite_nan() {
    struct NonFinite {
        value: f32,
    }

    impl NonFinite {
        fn to_bits(self) -> u32 {
            self.value.to_bits()
        }

        fn format_nonfinite(self) -> &'static str {
            const MANTISSA_MASK: u32 = 0x007fffff;
            const SIGN_MASK: u32 = 0x80000000;
            const NAN: &'static str = "NaN";
            const NEG_INFINITY: &'static str = "-Infinity";
            const INFINITY: &'static str = "Infinity";
            let bits = self.to_bits();
            if bits & MANTISSA_MASK != 0 {
                NAN
            } else if bits & SIGN_MASK != 0 {
                NEG_INFINITY
            } else {
                INFINITY
            }
        }
    }

    let non_finite_nan = NonFinite { value: f32::NAN };
    assert_eq!(non_finite_nan.format_nonfinite(), "NaN");
}

