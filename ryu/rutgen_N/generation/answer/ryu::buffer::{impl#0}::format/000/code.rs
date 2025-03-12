// Answer 0

#[test]
fn test_format_nan() {
    struct FloatNan;

    impl Float for FloatNan {
        fn is_nonfinite(&self) -> bool {
            true
        }

        fn format_nonfinite(&self) -> &str {
            "NaN"
        }
    }

    let mut buffer = String::new();
    let result = buffer.format(FloatNan);
    assert_eq!(result, "NaN");
}

#[test]
fn test_format_positive_infinity() {
    struct FloatPosInf;

    impl Float for FloatPosInf {
        fn is_nonfinite(&self) -> bool {
            true
        }

        fn format_nonfinite(&self) -> &str {
            "inf"
        }
    }

    let mut buffer = String::new();
    let result = buffer.format(FloatPosInf);
    assert_eq!(result, "inf");
}

#[test]
fn test_format_negative_infinity() {
    struct FloatNegInf;

    impl Float for FloatNegInf {
        fn is_nonfinite(&self) -> bool {
            true
        }

        fn format_nonfinite(&self) -> &str {
            "-inf"
        }
    }

    let mut buffer = String::new();
    let result = buffer.format(FloatNegInf);
    assert_eq!(result, "-inf");
}

#[test]
fn test_format_finite() {
    struct FloatFinite(f64);

    impl Float for FloatFinite {
        fn is_nonfinite(&self) -> bool {
            false
        }
    }

    impl FloatFinite {
        fn format_finite(&self) -> &str {
            &format!("{}", self.0)
        }
    }

    let mut buffer = String::new();
    let result = buffer.format(FloatFinite(3.14));
    assert_eq!(result, "3.14");
}

