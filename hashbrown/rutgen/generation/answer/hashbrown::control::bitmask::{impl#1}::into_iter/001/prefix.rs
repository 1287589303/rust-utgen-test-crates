// Answer 0

#[test]
fn test_into_iter_zero() {
    let bitmask = BitMask(0); // Fully zero
    let iter = bitmask.into_iter();
}

#[test]
fn test_into_iter_one() {
    let bitmask = BitMask(1); // Single bit set
    let iter = bitmask.into_iter();
}

#[test]
fn test_into_iter_boundary() {
    let bitmask = BitMask(BITMASK_MASK); // All bits set
    let iter = bitmask.into_iter();
}

#[test]
fn test_into_iter_power_of_two() {
    let bitmask = BitMask(2); // Single bit set (2^1)
    let iter = bitmask.into_iter();
}

#[test]
fn test_into_iter_power_of_two_five() {
    let bitmask = BitMask(16); // Single bit set (2^4)
    let iter = bitmask.into_iter();
}

#[test]
fn test_into_iter_power_of_two_seven() {
    let bitmask = BitMask(128); // Single bit set (2^7)
    let iter = bitmask.into_iter();
}

