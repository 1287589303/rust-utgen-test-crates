// Answer 0

#[test]
fn test_ceil_log2_pow5_zero() {
    let e = 0;
    let expected = log2_pow5(e) + 1;
    let result = ceil_log2_pow5(e);
    assert_eq!(result, expected);
}

#[test]
fn test_ceil_log2_pow5_positive() {
    let e = 10;
    let expected = log2_pow5(e) + 1;
    let result = ceil_log2_pow5(e);
    assert_eq!(result, expected);
}

#[test]
fn test_ceil_log2_pow5_boundary() {
    let e = 3528;
    let expected = log2_pow5(e) + 1;
    let result = ceil_log2_pow5(e);
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn test_ceil_log2_pow5_negative() {
    let e = -1;
    let _ = ceil_log2_pow5(e);
}

