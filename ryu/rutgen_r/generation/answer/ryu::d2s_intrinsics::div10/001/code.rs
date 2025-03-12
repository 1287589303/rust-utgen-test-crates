// Answer 0

#[test]
fn test_div10_zero() {
    let result = div10(0);
    assert_eq!(result, 0);
}

#[test]
fn test_div10_ten() {
    let result = div10(10);
    assert_eq!(result, 1);
}

#[test]
fn test_div10_fifty() {
    let result = div10(50);
    assert_eq!(result, 5);
}

#[test]
fn test_div10_one_hundred() {
    let result = div10(100);
    assert_eq!(result, 10);
}

#[test]
fn test_div10_large_number() {
    let result = div10(1_000_000);
    assert_eq!(result, 100_000);
}

#[test]
fn test_div10_edge_case() {
    let result = div10(9);
    assert_eq!(result, 0);
}

