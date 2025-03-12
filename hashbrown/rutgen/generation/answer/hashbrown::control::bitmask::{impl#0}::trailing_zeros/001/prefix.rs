// Answer 0

#[test]
fn test_trailing_zeros_zero() {
    let bitmask = BitMask(0);
    let result = bitmask.trailing_zeros();
}

#[test]
fn test_trailing_zeros_one() {
    let bitmask = BitMask(1);
    let result = bitmask.trailing_zeros();
}

#[test]
fn test_trailing_zeros_two() {
    let bitmask = BitMask(2);
    let result = bitmask.trailing_zeros();
}

#[test]
fn test_trailing_zeros_four() {
    let bitmask = BitMask(4);
    let result = bitmask.trailing_zeros();
}

#[test]
fn test_trailing_zeros_eight() {
    let bitmask = BitMask(8);
    let result = bitmask.trailing_zeros();
}

#[test]
fn test_trailing_zeros_maximum() {
    let bitmask = BitMask(u64::MAX); // Assuming BitMaskWord is u64
    let result = bitmask.trailing_zeros();
}

#[test]
fn test_trailing_zeros_with_minimum() {
    let bitmask = BitMask(0b1000_0000);
    let result = bitmask.trailing_zeros();
} 

#[test]
fn test_trailing_zeros_with_increasing_trailing_zeros() {
    let bitmask = BitMask(0b0001_1111); // 5 trailing zeros
    let result = bitmask.trailing_zeros();
}

#[test]
fn test_trailing_zeros_with_varying_trailing_zeros() {
    let bitmask = BitMask(0b0011_1111); // 6 trailing zeros
    let result = bitmask.trailing_zeros();
}

