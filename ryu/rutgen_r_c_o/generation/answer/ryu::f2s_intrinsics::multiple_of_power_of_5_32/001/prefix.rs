// Answer 0

#[test]
fn test_multiple_of_power_of_5_32_zero_value() {
    let value = 0;
    let p = 0;
    let _ = multiple_of_power_of_5_32(value, p);
}

#[test]
fn test_multiple_of_power_of_5_32_one_value() {
    let value = 1;
    let p = 0;
    let _ = multiple_of_power_of_5_32(value, p);
}

#[test]
fn test_multiple_of_power_of_5_32_five_value() {
    let value = 5;
    let p = 1;
    let _ = multiple_of_power_of_5_32(value, p);
}

#[test]
fn test_multiple_of_power_of_5_32_twenty_five_value() {
    let value = 25;
    let p = 2;
    let _ = multiple_of_power_of_5_32(value, p);
}

#[test]
fn test_multiple_of_power_of_5_32_one_hundred_twenty_five_value() {
    let value = 125;
    let p = 3;
    let _ = multiple_of_power_of_5_32(value, p);
}

#[test]
fn test_multiple_of_power_of_5_32_six_hundred_twenty_five_value() {
    let value = 625;
    let p = 4;
    let _ = multiple_of_power_of_5_32(value, p);
}

#[test]
fn test_multiple_of_power_of_5_32_boundary_cases() {
    let value = 5;
    let p = 5;
    let _ = multiple_of_power_of_5_32(value, p);
}

#[test]
fn test_multiple_of_power_of_5_32_large_case() {
    let value = 625;
    let p = 5;
    let _ = multiple_of_power_of_5_32(value, p);
}

