// Answer 0

#[should_panic]
fn test_log10_pow5_negative() {
    let _result = log10_pow5(-1);
}

#[should_panic]
fn test_log10_pow5_exceeds_upper_bound() {
    let _result = log10_pow5(2621);
}

