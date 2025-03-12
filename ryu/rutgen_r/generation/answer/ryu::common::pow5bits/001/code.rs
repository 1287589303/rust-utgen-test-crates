// Answer 0

#[test]
fn test_pow5bits_lower_bound() {
    let e: i32 = 0;
    let result = pow5bits(e);
    assert_eq!(result, 1);
}

#[test]
fn test_pow5bits_upper_bound() {
    let e: i32 = 3528;
    let result = pow5bits(e);
    assert_eq!(result, 575203);
}

