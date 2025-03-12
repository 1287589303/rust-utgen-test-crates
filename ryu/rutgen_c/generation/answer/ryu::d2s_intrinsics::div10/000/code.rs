// Answer 0

#[test]
fn test_div10_divisible_by_10() {
    let result = div10(100);
    assert_eq!(result, 10);
}

#[test]
fn test_div10_not_divisible_by_10() {
    let result = div10(99);
    assert_eq!(result, 9);
}

#[test]
fn test_div10_zero() {
    let result = div10(0);
    assert_eq!(result, 0);
}

#[test]
fn test_div10_large_number() {
    let result = div10(1_000_000);
    assert_eq!(result, 100_000);
}

#[test]
fn test_div10_boundary_case() {
    let result = div10(u64::MAX);
    assert_eq!(result, u64::MAX / 10);
}

