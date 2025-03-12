// Answer 0

#[test]
fn test_lowest_set_bit_case_1() {
    let bitmask = BitMask(1); // Lowest set bit at position 0
    let result = bitmask.lowest_set_bit();
}

#[test]
fn test_lowest_set_bit_case_2() {
    let bitmask = BitMask(2); // Lowest set bit at position 1
    let result = bitmask.lowest_set_bit();
}

#[test]
fn test_lowest_set_bit_case_3() {
    let bitmask = BitMask(4); // Lowest set bit at position 2
    let result = bitmask.lowest_set_bit();
}

#[test]
fn test_lowest_set_bit_case_4() {
    let bitmask = BitMask(8); // Lowest set bit at position 3
    let result = bitmask.lowest_set_bit();
}

#[test]
fn test_lowest_set_bit_case_5() {
    let bitmask = BitMask(15); // Lowest set bit at position 0 (multiple bits set)
    let result = bitmask.lowest_set_bit();
}

