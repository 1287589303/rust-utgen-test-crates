// Answer 0

#[test]
#[should_panic]
fn test_log10_pow2_negative_input() {
    log10_pow2(-1);
}

#[test]
#[should_panic]
fn test_log10_pow2_exceeds_upper_bound() {
    log10_pow2(1651);
}

