// Answer 0

#[test]
fn test_ceil_log2_pow5_zero() {
    let e = 0;
    let expected = log2_pow5(e) + 1;
    assert_eq!(ceil_log2_pow5(e), expected);
}

#[test]
fn test_ceil_log2_pow5_one() {
    let e = 1;
    let expected = log2_pow5(e) + 1;
    assert_eq!(ceil_log2_pow5(e), expected);
}

#[test]
fn test_ceil_log2_pow5_two() {
    let e = 2;
    let expected = log2_pow5(e) + 1;
    assert_eq!(ceil_log2_pow5(e), expected);
}

#[test]
fn test_ceil_log2_pow5_negative() {
    let e = -1;
    let expected = log2_pow5(e) + 1;
    assert_eq!(ceil_log2_pow5(e), expected);
}

#[test]
fn test_ceil_log2_pow5_large() {
    let e = 100;
    let expected = log2_pow5(e) + 1;
    assert_eq!(ceil_log2_pow5(e), expected);
}

