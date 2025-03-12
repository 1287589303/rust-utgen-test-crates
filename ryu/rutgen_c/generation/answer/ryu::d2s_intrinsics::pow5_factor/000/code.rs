// Answer 0

#[test]
fn test_pow5_factor_non_zero_input() {
    let result = pow5_factor(5);
    assert_eq!(result, 1);
}

#[test]
fn test_pow5_factor_large_value() {
    let result = pow5_factor(1_000_000);
    assert_eq!(result, 2);
}

#[test]
fn test_pow5_factor_boundary_value() {
    let result = pow5_factor(3689348814741910323);
    assert_eq!(result, 31); // This is an example assertion, value might vary based on context
}

#[should_panic]
fn test_pow5_factor_zero_input() {
    pow5_factor(0);
}

