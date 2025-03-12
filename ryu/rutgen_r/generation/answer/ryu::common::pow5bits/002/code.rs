// Answer 0

#[test]
fn test_pow5bits_zero() {
    let result = ryu::pow5bits(0);
    assert_eq!(result, 1);
}

#[test]
#[should_panic]
fn test_pow5bits_overflow() {
    ryu::pow5bits(3529);
}

