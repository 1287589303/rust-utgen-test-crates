// Answer 0

#[test]
fn test_insert_into_empty_pattern_set() {
    let capacity = 5;
    let mut pattern_set = PatternSet::new(capacity);
    let pid = PatternID(0);
    pattern_set.insert(pid);
}

#[test]
fn test_insert_multiple_pattern_ids() {
    let capacity = 5;
    let mut pattern_set = PatternSet::new(capacity);
    for id in 0..capacity {
        pattern_set.insert(PatternID(id));
    }
}

#[test]
fn test_insert_boundary_pattern_id() {
    let capacity = 5;
    let mut pattern_set = PatternSet::new(capacity);
    let pid = PatternID(capacity - 1);
    pattern_set.insert(pid);
}

#[test]
#[should_panic]
fn test_insert_pattern_id_exceeding_capacity() {
    let capacity = 5;
    let mut pattern_set = PatternSet::new(capacity);
    let pid = PatternID(capacity);
    pattern_set.insert(pid);
}

#[test]
fn test_insert_into_full_pattern_set() {
    let capacity = 5;
    let mut pattern_set = PatternSet::new(capacity);
    for id in 0..capacity {
        pattern_set.insert(PatternID(id));
    }
    let pid = PatternID(3); // This id is already inserted
    pattern_set.insert(pid);
}

