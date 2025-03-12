// Answer 0

#[test]
fn test_div5_positive() {
    let result = div5(10);
    assert_eq!(result, 2);
}

#[test]
fn test_div5_zero() {
    let result = div5(0);
    assert_eq!(result, 0);
}

#[test]
fn test_div5_large_number() {
    let result = div5(1_000_000);
    assert_eq!(result, 200_000);
}

#[test]
fn test_div5_remainder() {
    let result = div5(11);
    assert_eq!(result, 2);
}

#[test]
fn test_div5_boundary() {
    let result = div5(5);
    assert_eq!(result, 1);
}

