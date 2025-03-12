// Answer 0

#[should_panic]
fn test_log10_pow2_negative_e() {
    let result = log10_pow2(-1);
}

#[should_panic]
fn test_log10_pow2_negative_e_large() {
    let result = log10_pow2(-100);
}

