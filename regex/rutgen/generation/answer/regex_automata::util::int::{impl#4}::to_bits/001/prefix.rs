// Answer 0

#[test]
fn test_to_bits_minimum() {
    let value: i32 = -2_147_483_648;
    let result = value.to_bits();
}

#[test]
fn test_to_bits_maximum() {
    let value: i32 = 2_147_483_647;
    let result = value.to_bits();
}

#[test]
fn test_to_bits_zero() {
    let value: i32 = 0;
    let result = value.to_bits();
}

#[test]
fn test_to_bits_negative_one() {
    let value: i32 = -1;
    let result = value.to_bits();
}

#[test]
fn test_to_bits_negative_hundred() {
    let value: i32 = -100;
    let result = value.to_bits();
}

#[test]
fn test_to_bits_one() {
    let value: i32 = 1;
    let result = value.to_bits();
}

#[test]
fn test_to_bits_hundred() {
    let value: i32 = 100;
    let result = value.to_bits();
}

