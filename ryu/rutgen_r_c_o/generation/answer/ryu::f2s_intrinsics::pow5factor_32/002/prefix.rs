// Answer 0

#[test]
#[should_panic]
fn test_pow5factor_32_zero() {
    let value: u32 = 0;
    let _result = pow5factor_32(value);
}

#[test]
fn test_pow5factor_32_one() {
    let value: u32 = 1;
    let _result = pow5factor_32(value);
}

#[test]
fn test_pow5factor_32_two() {
    let value: u32 = 2;
    let _result = pow5factor_32(value);
}

#[test]
fn test_pow5factor_32_three() {
    let value: u32 = 3;
    let _result = pow5factor_32(value);
}

#[test]
fn test_pow5factor_32_four() {
    let value: u32 = 4;
    let _result = pow5factor_32(value);
}

#[test]
fn test_pow5factor_32_five() {
    let value: u32 = 5;
    let _result = pow5factor_32(value);
}

#[test]
fn test_pow5factor_32_ten() {
    let value: u32 = 10;
    let _result = pow5factor_32(value);
}

#[test]
fn test_pow5factor_32_twenty_five() {
    let value: u32 = 25;
    let _result = pow5factor_32(value);
}

#[test]
fn test_pow5factor_32_fifty() {
    let value: u32 = 50;
    let _result = pow5factor_32(value);
}

#[test]
fn test_pow5factor_32_one_hundred_twenty_five() {
    let value: u32 = 125;
    let _result = pow5factor_32(value);
}

#[test]
fn test_pow5factor_32_one_million() {
    let value: u32 = 1_000_000;
    let _result = pow5factor_32(value);
}

