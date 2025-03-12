// Answer 0

#[should_panic]
pub fn test_multiple_of_power_of_2_value_zero() {
    let value = 0; 
    let p = 1; 
    multiple_of_power_of_2(value, p);
}

#[should_panic]
pub fn test_multiple_of_power_of_2_value_zero_large_p() {
    let value = 0; 
    let p = 63; 
    multiple_of_power_of_2(value, p);
}

