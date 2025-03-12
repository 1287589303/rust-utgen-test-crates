// Answer 0

#[test]
fn test_pattern_set_new_zero_capacity() {
    let capacity = 0;
    let pattern_set = PatternSet::new(capacity);
}

#[test]
fn test_pattern_set_new_max_capacity() {
    let capacity = PatternID::LIMIT;
    let pattern_set = PatternSet::new(capacity);
}

#[test]
#[should_panic]
fn test_pattern_set_new_exceeding_capacity() {
    let capacity = PatternID::LIMIT + 1;
    let pattern_set = PatternSet::new(capacity);
}

