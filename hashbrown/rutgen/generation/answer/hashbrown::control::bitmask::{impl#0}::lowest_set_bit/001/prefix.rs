// Answer 0

#[test]
fn test_lowest_set_bit_case_1() {
    let bitmask = BitMask(1); // 0b0001
    let _ = bitmask.lowest_set_bit();
}

#[test]
fn test_lowest_set_bit_case_2() {
    let bitmask = BitMask(2); // 0b0010
    let _ = bitmask.lowest_set_bit();
}

#[test]
fn test_lowest_set_bit_case_3() {
    let bitmask = BitMask(3); // 0b0011
    let _ = bitmask.lowest_set_bit();
}

#[test]
fn test_lowest_set_bit_case_4() {
    let bitmask = BitMask(15); // 0b1111
    let _ = bitmask.lowest_set_bit();
}

#[test]
fn test_lowest_set_bit_case_5() {
    let bitmask = BitMask(0xFFFFFFFF); // All bits set
    let _ = bitmask.lowest_set_bit();
}

