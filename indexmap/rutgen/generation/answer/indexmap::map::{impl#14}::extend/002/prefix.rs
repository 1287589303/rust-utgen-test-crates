// Answer 0

#[test]
fn test_extend_non_empty_map_single_insert() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    map.extend(vec![(1, 100)]);
}

#[test]
fn test_extend_non_empty_map_multiple_inserts() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    map.extend(vec![(1, 100), (2, 200), (3, 300)]);
}

#[test]
fn test_extend_non_empty_map_with_duplicates() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    map.extend(vec![(1, 100), (2, 200), (1, 150), (3, 300)]);
}

#[test]
fn test_extend_non_empty_map_large_size() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    let items: Vec<(i32, i32)> = (1..=1000).map(|i| (i, i * 10)).collect();
    map.extend(items);
}

