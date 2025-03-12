// Answer 0

#[test]
fn test_invert_all_bits_cleared() {
    let input = BitMask(0);
    let result = input.invert();
}

#[test]
fn test_invert_all_bits_set() {
    let input = BitMask(BITMASK_MASK);
    let result = input.invert();
}

#[test]
fn test_invert_some_bits_set() {
    let input = BitMask(BITMASK_MASK >> 1); // example with some bits set
    let result = input.invert();
}

#[test]
fn test_invert_some_bits_set_2() {
    let input = BitMask(0b10101010); // example with specific bits set
    let result = input.invert();
}

#[test]
fn test_invert_edge_case() {
    let input = BitMask(1); // only least significant bit set
    let result = input.invert();
}

