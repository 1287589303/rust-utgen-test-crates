// Answer 0

#[test]
#[should_panic]
fn test_multiple_of_power_of_2_value_zero_p_0() {
    let value: u64 = 0;
    let p: u32 = 0;
    multiple_of_power_of_2(value, p);
}

#[test]
#[should_panic]
fn test_multiple_of_power_of_2_value_zero_p_1() {
    let value: u64 = 0;
    let p: u32 = 1;
    multiple_of_power_of_2(value, p);
}

#[test]
#[should_panic]
fn test_multiple_of_power_of_2_value_zero_p_62() {
    let value: u64 = 0;
    let p: u32 = 62;
    multiple_of_power_of_2(value, p);
}

#[test]
#[should_panic]
fn test_multiple_of_power_of_2_value_zero_p_63() {
    let value: u64 = 0;
    let p: u32 = 63;
    multiple_of_power_of_2(value, p);
}

