// Answer 0

#[test]
fn test_extend_with_single_pair() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_hasher(RandomState::new());
    let input = vec![(1, 10)];
    map.extend(input);
}

#[test]
fn test_extend_with_multiple_pairs() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_hasher(RandomState::new());
    let input = vec![(1, 10), (2, 20), (3, 30)];
    map.extend(input);
}

#[test]
fn test_extend_with_duplicates() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_hasher(RandomState::new());
    let input = vec![(1, 10), (1, 15), (2, 20)];
    map.extend(input);
}

#[test]
fn test_extend_with_empty_pairs() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_hasher(RandomState::new());
    let input: Vec<(i32, i32)> = vec![];
    map.extend(input);
}

#[test]
fn test_extend_with_large_capacity() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_hasher(RandomState::new());
    let input: Vec<(i32, i32)> = (1..1000).map(|i| (i, i * 10)).collect();
    map.extend(input);
}

