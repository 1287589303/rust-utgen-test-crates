// Answer 0

#[test]
fn test_is_nonfinite_with_infinity() {
    struct FloatWrapper(f64);
    
    impl FloatWrapper {
        fn to_bits(self) -> u64 {
            self.0.to_bits()
        }
        
        fn is_nonfinite(self) -> bool {
            const EXP_MASK: u64 = 0x7ff0000000000000;
            let bits = self.to_bits();
            bits & EXP_MASK == EXP_MASK
        }
    }

    let positive_infinity = FloatWrapper(f64::INFINITY);
    assert!(positive_infinity.is_nonfinite());

    let negative_infinity = FloatWrapper(f64::NEG_INFINITY);
    assert!(negative_infinity.is_nonfinite());
}

#[test]
fn test_is_nonfinite_with_nan() {
    struct FloatWrapper(f64);
    
    impl FloatWrapper {
        fn to_bits(self) -> u64 {
            self.0.to_bits()
        }
        
        fn is_nonfinite(self) -> bool {
            const EXP_MASK: u64 = 0x7ff0000000000000;
            let bits = self.to_bits();
            bits & EXP_MASK == EXP_MASK
        }
    }

    let nan = FloatWrapper(f64::NAN);
    assert!(!nan.is_nonfinite());
}

#[test]
fn test_is_nonfinite_with_finite_numbers() {
    struct FloatWrapper(f64);
    
    impl FloatWrapper {
        fn to_bits(self) -> u64 {
            self.0.to_bits()
        }
        
        fn is_nonfinite(self) -> bool {
            const EXP_MASK: u64 = 0x7ff0000000000000;
            let bits = self.to_bits();
            bits & EXP_MASK == EXP_MASK
        }
    }

    let finite_number = FloatWrapper(1.0);
    assert!(!finite_number.is_nonfinite());

    let small_finite_number = FloatWrapper(0.0001);
    assert!(!small_finite_number.is_nonfinite());

    let negative_finite_number = FloatWrapper(-1.0);
    assert!(!negative_finite_number.is_nonfinite());
}

