// Answer 0

#[should_panic]
fn test_pow5bits_negative_input() {
    let _ = pow5bits(-1);
}

#[should_panic]
fn test_pow5bits_boundary_input_below_zero() {
    let _ = pow5bits(-100);
}

