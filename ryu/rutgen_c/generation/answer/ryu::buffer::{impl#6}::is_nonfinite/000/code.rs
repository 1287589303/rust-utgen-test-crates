// Answer 0

#[test]
fn test_is_nonfinite_with_nonfinite_numbers() {
    let nonfinite_numbers = vec![f64::NAN, f64::INFINITY, f64::NEG_INFINITY];
    for &num in &nonfinite_numbers {
        assert!(num.is_nonfinite());
    }
}

#[test]
fn test_is_nonfinite_with_finite_numbers() {
    let finite_numbers = vec![0.0, 1.0, -1.0, 3.14159, -3.14159, 1e10, -1e10, 1.999999];
    for &num in &finite_numbers {
        assert!(!num.is_nonfinite());
    }
}

