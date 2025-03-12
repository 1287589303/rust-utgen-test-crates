// Answer 0

#[test]
fn test_next_with_one_set_bit() {
    let mut bitmask_iter = BitMaskIter(BitMask(1)); // binary: 0001
    let result = bitmask_iter.next();
}

#[test]
fn test_next_with_two_set_bits() {
    let mut bitmask_iter = BitMaskIter(BitMask(2)); // binary: 0010
    let result = bitmask_iter.next();
}

#[test]
fn test_next_with_three_set_bits() {
    let mut bitmask_iter = BitMaskIter(BitMask(3)); // binary: 0011
    let result = bitmask_iter.next();
}

#[test]
fn test_next_with_four_set_bits() {
    let mut bitmask_iter = BitMaskIter(BitMask(4)); // binary: 0100
    let result = bitmask_iter.next();
}

#[test]
fn test_next_with_five_set_bits() {
    let mut bitmask_iter = BitMaskIter(BitMask(5)); // binary: 0101
    let result = bitmask_iter.next();
}

#[test]
fn test_next_with_seven_set_bits() {
    let mut bitmask_iter = BitMaskIter(BitMask(7)); // binary: 0111
    let result = bitmask_iter.next();
}

#[test]
fn test_next_with_eight_set_bits() {
    let mut bitmask_iter = BitMaskIter(BitMask(8)); // binary: 1000
    let result = bitmask_iter.next();
}

#[test]
fn test_next_with_non_power_of_two() {
    let mut bitmask_iter = BitMaskIter(BitMask(11)); // binary: 1011
    let result = bitmask_iter.next();
}

