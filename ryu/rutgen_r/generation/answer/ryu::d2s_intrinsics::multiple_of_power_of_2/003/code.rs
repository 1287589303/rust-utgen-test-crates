// Answer 0

#[should_panic]
fn test_multiple_of_power_of_2_zero_value() {
    let value: u64 = 0; // This will trigger the debug assertion `value != 0`
    let p: u32 = 5;
    multiple_of_power_of_2(value, p);
}

#[should_panic]
fn test_multiple_of_power_of_2_negative_power() {
    let value: u64 = 8; // This is a valid non-zero value
    let p: u32 = 64; // This will trigger the debug assertion `p < 64`
    multiple_of_power_of_2(value, p);
}

