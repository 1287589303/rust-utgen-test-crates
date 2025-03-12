// Answer 0

#[test]
fn test_log10_pow5_lower_bound() {
    let result = ryu::log10_pow5(0);
    assert_eq!(result, 0);
}

#[should_panic]
fn test_log10_pow5_upper_bound() {
    ryu::log10_pow5(2621); // This should panic due to the second debug_assert.
}

