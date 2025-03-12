// Answer 0

#[test]
fn test_next_with_no_bits_set() {
    let bitmask = BitMask(0);
    let mut iterator = BitMaskIter(bitmask);
    let result = iterator.next();
}

#[test]
fn test_next_with_empty_bitmask() {
    let bitmask = BitMask(0);
    let mut iterator = BitMaskIter(bitmask);
    let result = iterator.next();
}

#[test]
fn test_next_calls_lowest_set_bit_none() {
    let bitmask = BitMask(0);
    let mut iterator = BitMaskIter(bitmask);
    let result = iterator.next();
}

