// Answer 0

#[test]
fn test_remove_lowest_bit_all_bits_clear() {
    let bitmask = BitMask(0);
    let result = bitmask.remove_lowest_bit();
}

#[test]
fn test_remove_lowest_bit_only_lowest_bit_set() {
    let bitmask = BitMask(1);
    let result = bitmask.remove_lowest_bit();
}

#[test]
fn test_remove_lowest_bit_power_of_two_1() {
    let bitmask = BitMask(2);
    let result = bitmask.remove_lowest_bit();
}

#[test]
fn test_remove_lowest_bit_power_of_two_2() {
    let bitmask = BitMask(4);
    let result = bitmask.remove_lowest_bit();
}

#[test]
fn test_remove_lowest_bit_power_of_two_3() {
    let bitmask = BitMask(8);
    let result = bitmask.remove_lowest_bit();
}

#[test]
fn test_remove_lowest_bit_non_power_of_two() {
    let bitmask = BitMask(6); // 110 in binary
    let result = bitmask.remove_lowest_bit();
}

#[test]
fn test_remove_lowest_bit_max_value() {
    let bitmask = BitMask(u64::MAX); // Assuming BitMaskWord can be u64
    let result = bitmask.remove_lowest_bit();
}

