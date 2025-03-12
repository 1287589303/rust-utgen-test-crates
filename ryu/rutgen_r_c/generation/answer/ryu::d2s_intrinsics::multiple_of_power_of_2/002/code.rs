// Answer 0

#[test]
#[should_panic]
fn test_multiple_of_power_of_2_p_equals_64() {
    let value: u64 = 8; // Example of a non-zero value
    let p: u32 = 64; // p should be equal to 64 to trigger the panic
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_valid_case() {
    let value: u64 = 8; // A non-zero value
    let p: u32 = 3; // A valid p within bounds
    assert!(multiple_of_power_of_2(value, p));
}

#[test]
#[should_panic]
fn test_multiple_of_power_of_2_zero_value() {
    let value: u64 = 0; // Zero value to trigger the panic on value
    let p: u32 = 5; // Valid p within bounds
    multiple_of_power_of_2(value, p);
}

