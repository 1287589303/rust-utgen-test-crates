// Answer 0

#[test]
fn truncate_empty_map() {
    let mut index_map = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    index_map.truncate(0);
}

#[test]
fn truncate_non_empty_map_exact_length() {
    let mut index_map = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    for i in 0..5 {
        index_map.insert(i, i);
    }
    index_map.truncate(5);
}

#[test]
fn truncate_non_empty_map_partial_length() {
    let mut index_map = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    for i in 0..5 {
        index_map.insert(i, i);
    }
    index_map.truncate(3);
}

#[test]
fn truncate_non_empty_map_exceed_length() {
    let mut index_map = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    for i in 0..5 {
        index_map.insert(i, i);
    }
    index_map.truncate(10);
}

#[test]
fn truncate_map_zero_length() {
    let mut index_map = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    for i in 0..5 {
        index_map.insert(i, i);
    }
    index_map.truncate(0);
}

