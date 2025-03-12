// Answer 0

#[test]
fn test_format_nonfinite_infinity() {
    struct FloatWrapper {
        value: f32,
    }

    impl FloatWrapper {
        fn to_bits(&self) -> u32 {
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
    
    let infinity_wrapper = FloatWrapper { value: f32::INFINITY };
    assert_eq!(infinity_wrapper.format_nonfinite(), "INFINITY");
}

#[test]
fn test_format_nonfinite_infinity_with_positive_zero() {
    struct FloatWrapper {
        value: f32,
    }

    impl FloatWrapper {
        fn to_bits(&self) -> u32 {
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
    
    let zero_wrapper = FloatWrapper { value: 0.0 };
    assert_eq!(zero_wrapper.format_nonfinite(), "INFINITY");
}

#[test]
fn test_format_nonfinite_infinity_with_negative_zero() {
    struct FloatWrapper {
        value: f32,
    }

    impl FloatWrapper {
        fn to_bits(&self) -> u32 {
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
    
    let negative_zero_wrapper = FloatWrapper { value: -0.0 };
    assert_eq!(negative_zero_wrapper.format_nonfinite(), "INFINITY");
}

