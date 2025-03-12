// Answer 0

#[test]
fn test_format_nan() {
    struct NaNFloat;

    impl Float for NaNFloat {
        fn is_nonfinite(&self) -> bool {
            true
        }

        fn format_nonfinite(&self) -> &str {
            "NaN"
        }
    }

    let mut buffer = String::new();
    let result = buffer.format(NaNFloat);
    assert_eq!(result, "NaN");
}

#[test]
fn test_format_positive_infinity() {
    struct PositiveInfinityFloat;

    impl Float for PositiveInfinityFloat {
        fn is_nonfinite(&self) -> bool {
            true
        }

        fn format_nonfinite(&self) -> &str {
            "inf"
        }
    }

    let mut buffer = String::new();
    let result = buffer.format(PositiveInfinityFloat);
    assert_eq!(result, "inf");
}

#[test]
fn test_format_negative_infinity() {
    struct NegativeInfinityFloat;

    impl Float for NegativeInfinityFloat {
        fn is_nonfinite(&self) -> bool {
            true
        }

        fn format_nonfinite(&self) -> &str {
            "-inf"
        }
    }

    let mut buffer = String::new();
    let result = buffer.format(NegativeInfinityFloat);
    assert_eq!(result, "-inf");
}

