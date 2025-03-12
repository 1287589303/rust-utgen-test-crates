// Answer 0

#[test]
fn test_pow5bits_zero() {
    assert_eq!(pow5bits(0), 1);
}

#[test]
fn test_pow5bits_mid() {
    assert_eq!(pow5bits(1), 5);
    assert_eq!(pow5bits(2), 25);
}

#[test]
fn test_pow5bits_boundary() {
    assert_eq!(pow5bits(3528), 1342177280); // corresponds to 5^3528
}

#[test]
#[should_panic]
fn test_pow5bits_below_zero() {
    pow5bits(-1);
}

#[test]
#[should_panic]
fn test_pow5bits_above_max() {
    pow5bits(3529);
}

