// Answer 0

#[should_panic]
fn test_pow5bits_negative() {
    let _ = pow5bits(-1);
}

#[should_panic]
fn test_pow5bits_overflow() {
    let _ = pow5bits(3529);
}

