// Answer 0

#[test]
#[should_panic]
fn test_log2_pow5_negative_input() {
    let e = -1; // This input violates the precondition (e >= 0)
    let result = log2_pow5(e);
}

#[test]
#[should_panic]
fn test_log2_pow5_over_upper_bound() {
    let e = 3529; // This input violates the precondition (e <= 3528)
    let result = log2_pow5(e);
}

