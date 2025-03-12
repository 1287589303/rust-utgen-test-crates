// Answer 0

#[test]
fn test_pow5_factor_boundary_below_n_div_5() {
    let value: u64 = 1; // value is non-zero and should produce a count of 0
    let result = pow5_factor(value);
    assert_eq!(result, 0);
}

#[test]
fn test_pow5_factor_boundary_n_div_5() {
    let value: u64 = 3689348814741910324; // value is just below N_DIV_5
    let result = pow5_factor(value);
    assert_eq!(result, 0);
}

#[test]
fn test_pow5_factor_boundary_above_n_div_5() {
    let value: u64 = 3689348814741910325; // value is just above N_DIV_5
    let result = pow5_factor(value);
    assert_eq!(result, 1); // first multiplication will exceed N_DIV_5
}

#[test]
fn test_pow5_factor_large_value() {
    let value: u64 = 3_000_000_000_000_000_000; // a large non-zero value
    let result = pow5_factor(value);
    assert!(result > 0); // this should produce a positive count
}

