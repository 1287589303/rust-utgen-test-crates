// Answer 0

#[test]
fn test_div5_zero() {
    let result = div5(0);
    assert_eq!(result, 0);
}

#[test]
fn test_div5_small_number() {
    let result = div5(1);
    assert_eq!(result, 0);
}

#[test]
fn test_div5_small_number_2() {
    let result = div5(4);
    assert_eq!(result, 0);
}

#[test]
fn test_div5_exact_division() {
    let result = div5(10);
    assert_eq!(result, 2);
}

#[test]
fn test_div5_large_number() {
    let result = div5(100);
    assert_eq!(result, 20);
}

#[test]
fn test_div5_large_number_2() {
    let result = div5(99);
    assert_eq!(result, 19);
}

#[test]
fn test_div5_boundary_number() {
    let result = div5(5);
    assert_eq!(result, 1);
}

#[test]
fn test_div5_large_power_of_five() {
    let result = div5(625);
    assert_eq!(result, 125);
}

