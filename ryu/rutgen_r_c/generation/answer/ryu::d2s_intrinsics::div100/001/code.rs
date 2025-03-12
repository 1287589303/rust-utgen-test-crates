// Answer 0

#[test]
fn test_div100_zero() {
    let result = div100(0);
    assert_eq!(result, 0);
}

#[test]
fn test_div100_less_than_100() {
    let result = div100(99);
    assert_eq!(result, 0);
}

#[test]
fn test_div100_exact_100() {
    let result = div100(100);
    assert_eq!(result, 1);
}

#[test]
fn test_div100_more_than_100() {
    let result = div100(250);
    assert_eq!(result, 2);
}

#[test]
fn test_div100_large_number() {
    let result = div100(10_000);
    assert_eq!(result, 100);
}

#[test]
fn test_div100_boundary_large() {
    let result = div100(u64::MAX);
    assert_eq!(result, u64::MAX / 100);
}

