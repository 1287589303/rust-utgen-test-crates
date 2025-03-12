// Answer 0

#[test]
fn test_next_empty_lookset() {
    let mut iter = LookSetIter { set: LookSet::empty() };
    let result = iter.next();
}

#[test]
fn test_next_with_zero_bits() {
    let mut iter = LookSetIter { set: LookSet { bits: 0 } };
    let result = iter.next();
}

