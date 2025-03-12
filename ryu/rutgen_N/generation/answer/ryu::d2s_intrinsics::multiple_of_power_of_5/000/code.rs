// Answer 0

#[test]
fn test_multiple_of_power_of_5_zero() {
    assert_eq!(ryu::multiple_of_power_of_5(1, 0), true);
}

#[test]
fn test_multiple_of_power_of_5_one() {
    assert_eq!(ryu::multiple_of_power_of_5(5, 1), true);
}

#[test]
fn test_multiple_of_power_of_5_two() {
    assert_eq!(ryu::multiple_of_power_of_5(25, 2), true);
}

#[test]
fn test_multiple_of_power_of_5_three() {
    assert_eq!(ryu::multiple_of_power_of_5(125, 3), true);
}

#[test]
fn test_multiple_of_power_of_5_not_enough() {
    assert_eq!(ryu::multiple_of_power_of_5(10, 2), false);
}

#[test]
fn test_multiple_of_power_of_5_large_number() {
    assert_eq!(ryu::multiple_of_power_of_5(3125, 5), true);
}

#[test]
fn test_multiple_of_power_of_5_low_value() {
    assert_eq!(ryu::multiple_of_power_of_5(0, 0), true);
}

