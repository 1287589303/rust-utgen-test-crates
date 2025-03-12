// Answer 0

#[test]
fn test_is_nonfinite_positive_infinity() {
    struct TestFloat {
        value: f32,
    }

    impl TestFloat {
        fn to_bits(self) -> u32 {
            self.value.to_bits()
        }
    }

    let positive_infinity = TestFloat { value: std::f32::INFINITY };
    assert!(positive_infinity.is_nonfinite());
}

#[test]
fn test_is_nonfinite_negative_infinity() {
    struct TestFloat {
        value: f32,
    }

    impl TestFloat {
        fn to_bits(self) -> u32 {
            self.value.to_bits()
        }
    }

    let negative_infinity = TestFloat { value: std::f32::NEG_INFINITY };
    assert!(negative_infinity.is_nonfinite());
}

#[test]
fn test_is_nonfinite_nan() {
    struct TestFloat {
        value: f32,
    }

    impl TestFloat {
        fn to_bits(self) -> u32 {
            self.value.to_bits()
        }
    }

    let nan_value = TestFloat { value: std::f32::NAN };
    assert!(!nan_value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_normal_value() {
    struct TestFloat {
        value: f32,
    }

    impl TestFloat {
        fn to_bits(self) -> u32 {
            self.value.to_bits()
        }
    }

    let normal_value = TestFloat { value: 1.0 };
    assert!(!normal_value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_zero() {
    struct TestFloat {
        value: f32,
    }

    impl TestFloat {
        fn to_bits(self) -> u32 {
            self.value.to_bits()
        }
    }

    let zero_value = TestFloat { value: 0.0 };
    assert!(!zero_value.is_nonfinite());
}

