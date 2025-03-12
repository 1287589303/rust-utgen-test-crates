// Answer 0

#[test]
fn test_pow5factor_32_basic() {
    assert_eq!(pow5factor_32(25), 2); // 25 = 5^2
    assert_eq!(pow5factor_32(125), 3); // 125 = 5^3
    assert_eq!(pow5factor_32(0), 0); // Edge case (though not specified to test zero)
}

#[test]
#[should_panic]
fn test_pow5factor_32_zero() {
    pow5factor_32(0); // This should panic due to debug assert
}

#[test]
fn test_pow5factor_32_non_multiple_of_five() {
    assert_eq!(pow5factor_32(1), 0);
    assert_eq!(pow5factor_32(3), 0);
    assert_eq!(pow5factor_32(9), 0);
}

#[test]
fn test_pow5factor_32_boundary_conditions() {
    assert_eq!(pow5factor_32(5), 1); // 5 = 5^1
    assert_eq!(pow5factor_32(10), 1); // 10 = 5^1 * 2
    assert_eq!(pow5factor_32(50), 2); // 50 = 5^2 * 2
}

