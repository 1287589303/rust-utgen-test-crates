// Answer 0

#[test]
fn test_get_range_inclusive_start_exclusive_end() {
    let mut index_set = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);
    let result = index_set.get_range(0..2);
}

#[test]
fn test_get_range_inclusive_start_inclusive_end() {
    let mut index_set = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);
    let result = index_set.get_range(0..=2);
}

#[test]
fn test_get_range_exclusive_start_inclusive_end() {
    let mut index_set = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);
    let result = index_set.get_range(1..=2);
}

#[test]
fn test_get_range_exclusive_start_exclusive_end() {
    let mut index_set = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);
    let result = index_set.get_range(0..2);
}

#[test]
fn test_get_range_full_range() {
    let mut index_set = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);
    let result = index_set.get_range(0..3);
}

