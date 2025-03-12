// Answer 0

#[test]
fn test_any_bit_set_zero() {
    let bitmask = BitMask(0);
    let _result = bitmask.any_bit_set();
}

#[test]
fn test_any_bit_set_non_zero() {
    let bitmask = BitMask(1);
    let _result = bitmask.any_bit_set();
}

#[test]
fn test_any_bit_set_max() {
    let bitmask = BitMask(u32::MAX);
    let _result = bitmask.any_bit_set();
} 

#[test]
fn test_any_bit_set_middle() {
    let bitmask = BitMask(0b101010);
    let _result = bitmask.any_bit_set();
} 

#[test]
fn test_any_bit_set_highest_bit() {
    let bitmask = BitMask(0b10000000);
    let _result = bitmask.any_bit_set();
}

