// Answer 0

#[test]
fn test_pow5bits_e_is_0() {
    let result = pow5bits(0);
}

#[test]
fn test_pow5bits_e_is_1() {
    let result = pow5bits(1);
}

#[test]
fn test_pow5bits_e_is_3528() {
    let result = pow5bits(3528);
}

#[should_panic]
#[test]
fn test_pow5bits_e_is_3529() {
    let result = pow5bits(3529);
}

