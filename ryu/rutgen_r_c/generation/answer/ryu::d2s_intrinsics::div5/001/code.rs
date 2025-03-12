// Answer 0

#[test]
fn test_div5_zero() {
    let result = div5(0);
    assert_eq!(result, 0);
}

#[test]
fn test_div5_five() {
    let result = div5(5);
    assert_eq!(result, 1);
}

#[test]
fn test_div5_ten() {
    let result = div5(10);
    assert_eq!(result, 2);
}

#[test]
fn test_div5_fifteen() {
    let result = div5(15);
    assert_eq!(result, 3);
}

#[test]
fn test_div5_twenty() {
    let result = div5(20);
    assert_eq!(result, 4);
}

#[test]
fn test_div5_large_number() {
    let result = div5(1_000_000);
    assert_eq!(result, 200_000);
}

#[test]
fn test_div5_large_number_non_divisible() {
    let result = div5(1_000_003);
    assert_eq!(result, 200_000);
}

