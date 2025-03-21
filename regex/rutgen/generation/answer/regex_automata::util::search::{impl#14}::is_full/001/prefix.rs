// Answer 0

#[cfg(test)]
fn test_pattern_set_is_full_zero_capacity() {
    let capacity = 0;
    let mut pattern_set = PatternSet::new(capacity);
    assert_eq!(pattern_set.is_full(), true);
}

#[cfg(test)]
fn test_pattern_set_is_full_one_capacity() {
    let capacity = 1;
    let mut pattern_set = PatternSet::new(capacity);
    assert_eq!(pattern_set.is_full(), false);
    pattern_set.insert(PatternID::new(0));
    assert_eq!(pattern_set.is_full(), true);
}

#[cfg(test)]
fn test_pattern_set_is_full_multiple_capacity() {
    let capacity = 5;
    let mut pattern_set = PatternSet::new(capacity);
    assert_eq!(pattern_set.is_full(), false);
    for pid in 0..capacity {
        pattern_set.insert(PatternID::new(pid));
    }
    assert_eq!(pattern_set.is_full(), true);
}

#[cfg(test)]
fn test_pattern_set_is_full_partial_capacity() {
    let capacity = 5;
    let mut pattern_set = PatternSet::new(capacity);
    assert_eq!(pattern_set.is_full(), false);
    for pid in 0..capacity - 1 {
        pattern_set.insert(PatternID::new(pid));
    }
    assert_eq!(pattern_set.is_full(), false);
}

