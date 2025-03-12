// Answer 0

#[test]
fn test_pow5factor_32_non_zero_values() {
    assert_eq!(pow5factor_32(5), 1);
    assert_eq!(pow5factor_32(25), 2);
    assert_eq!(pow5factor_32(125), 3);
    assert_eq!(pow5factor_32(1), 0);
}

#[test]
#[should_panic]
fn test_pow5factor_32_zero_value() {
    let _ = pow5factor_32(0);
}

