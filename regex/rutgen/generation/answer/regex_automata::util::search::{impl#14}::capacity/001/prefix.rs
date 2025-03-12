// Answer 0

#[test]
fn test_capacity_empty() {
    let pattern_set = PatternSet::new(0);
    let capacity = pattern_set.capacity();
}

#[test]
fn test_capacity_partial() {
    let pattern_set = PatternSet::new(5);
    let capacity = pattern_set.capacity();
}

#[test]
fn test_capacity_full() {
    let pattern_set = PatternSet::new(PatternID::LIMIT);
    let capacity = pattern_set.capacity();
}

#[test]
fn test_capacity_beyond_limit() {
    let pattern_set = PatternSet::new(PatternID::LIMIT + 1);
    let capacity = pattern_set.capacity();
}

