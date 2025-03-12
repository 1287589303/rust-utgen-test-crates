// Answer 0

#[test]
fn test_format_nonfinite_nan() {
    struct TestValue {
        value: f64,
    }

    impl TestValue {
        fn to_bits(self) -> u64 {
            self.value.to_bits()
        }
    }

    let test_value = TestValue { value: f64::NAN };
    assert_eq!(test_value.format_nonfinite(), "NAN");
}

#[test]
fn test_format_nonfinite_negative_infinity() {
    struct TestValue {
        value: f64,
    }

    impl TestValue {
        fn to_bits(self) -> u64 {
            self.value.to_bits()
        }
    }

    let test_value = TestValue { value: f64::NEG_INFINITY };
    assert_eq!(test_value.format_nonfinite(), "NEG_INFINITY");
}

#[test]
fn test_format_nonfinite_infinity() {
    struct TestValue {
        value: f64,
    }

    impl TestValue {
        fn to_bits(self) -> u64 {
            self.value.to_bits()
        }
    }

    let test_value = TestValue { value: f64::INFINITY };
    assert_eq!(test_value.format_nonfinite(), "INFINITY");
}

#[test]
fn test_format_nonfinite_normal() {
    struct TestValue {
        value: f64,
    }

    impl TestValue {
        fn to_bits(self) -> u64 {
            self.value.to_bits()
        }
    }

    let test_value = TestValue { value: 1.0 };
    assert_eq!(test_value.format_nonfinite(), "INFINITY"); // Assuming the function is supposed to behave this way
}

