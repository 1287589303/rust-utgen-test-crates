// Answer 0

#[should_panic]
fn test_log2_pow5_negative_input() {
    let _ = log2_pow5(-1);
}

#[should_panic]
fn test_log2_pow5_overflow_input() {
    let _ = log2_pow5(3529);
}

