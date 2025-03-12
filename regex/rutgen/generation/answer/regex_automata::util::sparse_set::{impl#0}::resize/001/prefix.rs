// Answer 0

#[test]
fn test_resize_to_zero() {
    let mut sparse_sets = SparseSets::new(StateID::LIMIT);
    sparse_sets.resize(0);
}

#[test]
fn test_resize_to_half_limit() {
    let mut sparse_sets = SparseSets::new(StateID::LIMIT);
    sparse_sets.resize(StateID::LIMIT / 2);
}

#[test]
fn test_resize_to_limit() {
    let mut sparse_sets = SparseSets::new(StateID::LIMIT);
    sparse_sets.resize(StateID::LIMIT);
}

#[test] #[should_panic]
fn test_resize_above_limit() {
    let mut sparse_sets = SparseSets::new(StateID::LIMIT);
    sparse_sets.resize(StateID::LIMIT + 1);
}

