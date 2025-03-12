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

    let nan_value = NonFinite { value: f32::NAN };
    assert_eq!(nan_value.format_nonfinite(), "NAN");
}

#[test]
fn test_format_nonfinite_negative_infinity() {
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

    let neg_infinity_value = NonFinite { value: f32::NEG_INFINITY };
    assert_eq!(neg_infinity_value.format_nonfinite(), "NEG_INFINITY");
}

#[test]
fn test_format_nonfinite_infinity() {
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

    let infinity_value = NonFinite { value: f32::INFINITY };
    assert_eq!(infinity_value.format_nonfinite(), "INFINITY");
}

