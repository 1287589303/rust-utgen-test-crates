// Answer 0

#[test]
fn test_next_non_empty_set_with_invalid_bits_1() {
    let mut iter = LookSetIter {
        set: LookSet { bits: 0b0000_0000_0000_0000_0100_0000 }, // 64, not defined in Look
    };
    let _ = iter.next();
}

#[test]
fn test_next_non_empty_set_with_invalid_bits_2() {
    let mut iter = LookSetIter {
        set: LookSet { bits: 0b0000_0000_0000_0000_0110_0000 }, // 96, not defined in Look
    };
    let _ = iter.next();
}

#[test]
fn test_next_non_empty_set_with_invalid_bits_3() {
    let mut iter = LookSetIter {
        set: LookSet { bits: 0b0000_0000_0000_0000_1000_0000 }, // 128, not defined in Look
    };
    let _ = iter.next();
}

#[test]
fn test_next_non_empty_set_with_invalid_bits_4() {
    let mut iter = LookSetIter {
        set: LookSet { bits: 0b0000_0000_0000_0001_0000_0000 }, // 256, not defined in Look
    };
    let _ = iter.next();
}

#[test]
fn test_next_non_empty_set_with_invalid_bits_5() {
    let mut iter = LookSetIter {
        set: LookSet { bits: 0b0000_0000_0000_0000_0000_0010 }, // 2, not defined in Look
    };
    let _ = iter.next();
}

