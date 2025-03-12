// Answer 0

#[test]
#[should_panic]
fn test_pow5factor_32_zero() {
    // Call the function with a value that causes it to panic due to the debug_assert.
    let _ = pow5factor_32(0);
}

#[test]
fn test_pow5factor_32_non_zero() {
    // Call the function with a non-zero value.
    let result = pow5factor_32(25); // 25 = 5^2, should return 2
    assert_eq!(result, 2);

    let result = pow5factor_32(125); // 125 = 5^3, should return 3
    assert_eq!(result, 3);

    let result = pow5factor_32(60); // 60 = 2^2 * 3 * 5, should return 1
    assert_eq!(result, 1);
}

