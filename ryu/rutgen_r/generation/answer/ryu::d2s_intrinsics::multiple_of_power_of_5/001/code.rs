// Answer 0

#[test]
fn test_multiple_of_power_of_5_zero() {
    let value = 0;
    let p = 0;
    assert!(ryu::multiple_of_power_of_5(value, p));
}

#[test]
fn test_multiple_of_power_of_5_non_multiple() {
    let value = 3;
    let p = 1;
    assert!(!ryu::multiple_of_power_of_5(value, p));
}

#[test]
fn test_multiple_of_power_of_5_one() {
    let value = 5;
    let p = 1;
    assert!(ryu::multiple_of_power_of_5(value, p));
}

#[test]
fn test_multiple_of_power_of_5_multiple() {
    let value = 25; 
    let p = 2;
    assert!(ryu::multiple_of_power_of_5(value, p));
}

#[test]
fn test_multiple_of_power_of_5_higher_power() {
    let value = 125; 
    let p = 3;
    assert!(ryu::multiple_of_power_of_5(value, p));
}

#[test]
fn test_multiple_of_power_of_5_exceeding_power() {
    let value = 25; 
    let p = 3;
    assert!(!ryu::multiple_of_power_of_5(value, p));
}

