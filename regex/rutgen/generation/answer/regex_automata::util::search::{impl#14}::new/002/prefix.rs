// Answer 0

#[test]
#[should_panic]
fn test_pattern_set_new_exceeds_limit() {
    let capacity = PatternID::LIMIT + 1;
    let _pattern_set = PatternSet::new(capacity);
}

