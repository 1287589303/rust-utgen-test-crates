// Answer 0

#[test]
fn test_is_nonfinite_with_nonfinite_value() {
    struct FloatWrapper {
        value: f32,
    }

    impl FloatWrapper {
        fn to_bits(self) -> u32 {
            self.value.to_bits()
        }

        fn is_nonfinite(self) -> bool {
            const EXP_MASK: u32 = 0x7f800000;
            let bits = self.to_bits();
            bits & EXP_MASK == EXP_MASK
        }
    }

    let non_finite_value = FloatWrapper { value: f32::INFINITY };
    assert!(non_finite_value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_with_finite_value() {
    struct FloatWrapper {
        value: f32,
    }

    impl FloatWrapper {
        fn to_bits(self) -> u32 {
            self.value.to_bits()
        }

        fn is_nonfinite(self) -> bool {
            const EXP_MASK: u32 = 0x7f800000;
            let bits = self.to_bits();
            bits & EXP_MASK == EXP_MASK
        }
    }

    let finite_value = FloatWrapper { value: 3.14 };
    assert!(!finite_value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_with_nan_value() {
    struct FloatWrapper {
        value: f32,
    }

    impl FloatWrapper {
        fn to_bits(self) -> u32 {
            self.value.to_bits()
        }

        fn is_nonfinite(self) -> bool {
            const EXP_MASK: u32 = 0x7f800000;
            let bits = self.to_bits();
            bits & EXP_MASK == EXP_MASK
        }
    }

    let nan_value = FloatWrapper { value: f32::NAN };
    assert!(!nan_value.is_nonfinite());
}

