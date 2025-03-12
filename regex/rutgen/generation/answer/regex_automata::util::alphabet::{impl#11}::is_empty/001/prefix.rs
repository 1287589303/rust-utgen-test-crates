// Answer 0

#[test]
fn test_is_empty_when_bits_are_zero() {
    let byte_set = ByteSet { bits: BitSet([0, 0]) };
    byte_set.is_empty();
}

#[test]
fn test_is_empty_when_bits_are_non_zero() {
    let byte_set = ByteSet { bits: BitSet([1, 0]) };
    byte_set.is_empty();
}

#[test]
fn test_is_empty_when_bits_are_non_zero2() {
    let byte_set = ByteSet { bits: BitSet([0, 1]) };
    byte_set.is_empty();
}

#[test]
fn test_is_empty_when_bits_are_non_zero3() {
    let byte_set = ByteSet { bits: BitSet([1, 1]) };
    byte_set.is_empty();
}

