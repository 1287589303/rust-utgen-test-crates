// Answer 0

#[test]
#[should_panic]
fn test_pow5bits_negative_input() {
    let e: i32 = -1;
    let result = pow5bits(e);
}

#[test]
#[should_panic]
fn test_pow5bits_input_exceeding_upper_bound() {
    let e: i32 = 3529;
    let result = pow5bits(e);
}

#[test]
#[should_panic]
fn test_pow5bits_large_negative_input() {
    let e: i32 = -1000;
    let result = pow5bits(e);
}

