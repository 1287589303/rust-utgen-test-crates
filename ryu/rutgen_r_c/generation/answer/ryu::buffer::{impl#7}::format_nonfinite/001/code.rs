// Answer 0

#[test]
fn test_format_nonfinite_nan() {
    struct NaNWrapper(f64);
    impl Copy for NaNWrapper {}

    let nan_value = NaNWrapper(f64::NAN);
    let result = nan_value.format_nonfinite();
    assert_eq!(result, "NaN");
}

#[test]
fn test_format_nonfinite_negative_infinity() {
    struct NegInfinityWrapper(f64);
    impl Copy for NegInfinityWrapper {}

    let neg_infinity_value = NegInfinityWrapper(f64::NEG_INFINITY);
    let result = neg_infinity_value.format_nonfinite();
    assert_eq!(result, "-inf");
}

#[test]
fn test_format_nonfinite_infinity() {
    struct InfinityWrapper(f64);
    impl Copy for InfinityWrapper {}

    let infinity_value = InfinityWrapper(f64::INFINITY);
    let result = infinity_value.format_nonfinite();
    assert_eq!(result, "inf");
}

