// Answer 0

#[test]
fn test_log10_pow2_min_boundary() {
    let result = log10_pow2(0);
    assert_eq!(result, 0);
}

#[test]
fn test_log10_pow2_mid_value() {
    let result = log10_pow2(1000);
    assert_eq!(result, 442);
}

#[test]
fn test_log10_pow2_max_boundary() {
    let result = log10_pow2(1650);
    assert_eq!(result, 754);
}

#[should_panic]
fn test_log10_pow2_below_min_boundary() {
    let _ = log10_pow2(-1);
}

#[should_panic]
fn test_log10_pow2_above_max_boundary() {
    let _ = log10_pow2(1651);
}

