// Answer 0

#[derive(Debug)]
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

#[test]
fn test_is_nonfinite_infinity() {
    let value = FloatWrapper(f64::INFINITY);
    assert!(value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_nan() {
    let value = FloatWrapper(f64::NAN);
    assert!(value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_finite() {
    let value = FloatWrapper(1.0);
    assert!(!value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_negative_infinity() {
    let value = FloatWrapper(f64::NEG_INFINITY);
    assert!(value.is_nonfinite());
}

