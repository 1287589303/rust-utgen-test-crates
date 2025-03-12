// Answer 0

#[test]
fn test_div100_small_value() {
    let x: u64 = 150;
    assert_eq!(div100(x), 1);
}

#[test]
fn test_div100_exact_hundred() {
    let x: u64 = 100;
    assert_eq!(div100(x), 1);
}

#[test]
fn test_div100_multiple_of_hundred() {
    let x: u64 = 500;
    assert_eq!(div100(x), 5);
}

#[test]
fn test_div100_zero() {
    let x: u64 = 0;
    assert_eq!(div100(x), 0);
}

#[test]
fn test_div100_large_value() {
    let x: u64 = 10000;
    assert_eq!(div100(x), 100);
}

#[test]
fn test_div100_boundary_value() {
    let x: u64 = 99;
    assert_eq!(div100(x), 0);
}

