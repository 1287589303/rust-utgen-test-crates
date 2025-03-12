// Answer 0

#[test]
fn test_next_with_b_at_upper_bound() {
    let mut byte_set = ByteSet::default();
    byte_set.add(255);
    
    let mut iter = ByteSetIter { set: &byte_set, b: 255 };
    let result = iter.next();
}

#[test]
fn test_next_with_b_at_upper_bound_contains() {
    let mut byte_set = ByteSet::default();
    byte_set.add(254);
    byte_set.add(255);
    
    let mut iter = ByteSetIter { set: &byte_set, b: 255 };
    let result = iter.next();
}

#[test]
fn test_next_with_b_at_upper_bound_only_empty() {
    let mut byte_set = ByteSet::default();
    
    let mut iter = ByteSetIter { set: &byte_set, b: 255 };
    let result = iter.next();
}

