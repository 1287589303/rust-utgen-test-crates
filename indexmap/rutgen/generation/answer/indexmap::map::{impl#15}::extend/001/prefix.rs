// Answer 0

#[test]
fn test_extend_empty_iterable() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::default();
    let empty_iter: Vec<(&i32, &i32)> = vec![];
    map.extend(empty_iter);
}

#[test]
fn test_extend_with_duplicate_keys() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::default();
    let duplicate_iter: Vec<(&i32, &i32)> = vec![(&1, &10), (&1, &20)];
    map.extend(duplicate_iter);
}

#[test]
fn test_extend_with_non_unique_keys() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::default();
    let non_unique_iter: Vec<(&i32, &i32)> = vec![(&1, &10), (&2, &20), (&1, &30)];
    map.extend(non_unique_iter);
}

#[test]
fn test_extend_large_iterable() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::default();
    let large_iter: Vec<(&i32, &i32)> = (0..1000).map(|i| (&i, &(i * 10))).collect();
    map.extend(large_iter);
}

#[test]
fn test_extend_mixed_items() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::default();
    let mixed_iter: Vec<(&i32, &i32)> = vec![(&1, &10), (&2, &20), (&3, &30), (&2, &25)];
    map.extend(mixed_iter);
}

