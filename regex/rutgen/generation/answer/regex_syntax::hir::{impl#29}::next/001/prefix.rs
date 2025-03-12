// Answer 0

#[test]
fn test_next_with_empty_lookset() {
    let mut iter = LookSetIter { set: LookSet::empty() };
    let result = iter.next();
}

#[test]
fn test_next_with_all_bits_zero() {
    let mut iter = LookSetIter { set: LookSet { bits: 0 } };
    let result = iter.next();
}

