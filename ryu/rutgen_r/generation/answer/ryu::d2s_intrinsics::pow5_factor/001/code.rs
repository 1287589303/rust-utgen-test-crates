// Answer 0

#[test]
fn test_pow5_factor_small_value() {
    let value: u64 = 1; // value != 0
    let result = pow5_factor(value);
    assert_eq!(result, 0);
}

#[test]
fn test_pow5_factor_boundary_value() {
    let value: u64 = 3689348814741910324; // value > N_DIV_5
    let result = pow5_factor(value);
    assert!(result > 0);
}

#[test]
fn test_pow5_factor_large_value() {
    let value: u64 = 10000; // value != 0
    let result = pow5_factor(value);
    assert_eq!(result, 0); // Should still be within the limit
}

