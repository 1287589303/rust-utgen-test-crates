// Answer 0

#[test]
fn test_multiple_of_power_of_5_true() {
    let value = 125; // 5^3
    let p = 3;
    assert!(multiple_of_power_of_5(value, p));
}

#[test]
fn test_multiple_of_power_of_5_false() {
    let value = 24; // Not a multiple of 5^1 or greater
    let p = 1;
    assert!(!multiple_of_power_of_5(value, p));
}

#[test]
fn test_multiple_of_power_of_5_edge_case() {
    let value = 5; // 5^1
    let p = 1;
    assert!(multiple_of_power_of_5(value, p));
}

#[test]
fn test_multiple_of_power_of_5_zero() {
    let value = 0; // 0 case
    let p = 1;
    assert!(!multiple_of_power_of_5(value, p));
}

#[test]
fn test_multiple_of_power_of_5_large_value() {
    let value = 3125; // 5^5
    let p = 5;
    assert!(multiple_of_power_of_5(value, p));
}

#[test]
fn test_multiple_of_power_of_5_large_value_false() {
    let value = 3124; // Just below 5^5
    let p = 5;
    assert!(!multiple_of_power_of_5(value, p));
}

