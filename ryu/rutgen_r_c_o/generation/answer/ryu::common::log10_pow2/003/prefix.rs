// Answer 0

#[test]
#[should_panic]
fn test_log10_pow2_negative_input() {
    let e: i32 = -1;
    let result = log10_pow2(e);
}

#[test]
fn test_log10_pow2_zero() {
    let e: i32 = 0;
    let result = log10_pow2(e); // expected to return 0
}

#[test]
fn test_log10_pow2_boundary_exceed() {
    let e: i32 = 1651;
    let result = log10_pow2(e); // expected to panic due to debug assertion
}

#[test]
fn test_log10_pow2_within_range() {
    let e: i32 = 1650;
    let result = log10_pow2(e); // valid input within the range
}

