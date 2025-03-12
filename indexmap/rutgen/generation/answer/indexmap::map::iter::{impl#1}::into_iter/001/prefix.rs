// Answer 0

#[test]
fn test_into_iter_empty() {
    let index_map: super::IndexMap<i32, i32, _> = super::IndexMap::with_hasher(Default::default());
    let _iter = index_map.into_iter();
}

#[test]
fn test_into_iter_single_element() {
    let mut index_map: super::IndexMap<i32, i32, _> = super::IndexMap::with_capacity_and_hasher(1, Default::default());
    index_map.insert(1, 100);
    let _iter = index_map.into_iter();
}

#[test]
fn test_into_iter_multiple_elements() {
    let mut index_map: super::IndexMap<i32, i32, _> = super::IndexMap::with_capacity_and_hasher(5, Default::default());
    index_map.insert(1, 100);
    index_map.insert(2, 200);
    let _iter = index_map.into_iter();
}

#[test]
fn test_into_iter_large_capacity() {
    let mut index_map: super::IndexMap<i32, i32, _> = super::IndexMap::with_capacity_and_hasher(1000, Default::default());
    for i in 0..100 {
        index_map.insert(i, i * 10);
    }
    let _iter = index_map.into_iter();
}

