// Answer 0

#[test]
fn test_get_index_valid_start() {
    let mut index_set: super::IndexSet<i32, ()> = super::IndexSet {
        map: super::IndexMap::new(),
    };
    index_set.map.core.push(super::Bucket { hash: 0, key: 1, value: () });
    let _ = index_set.get_index(0);
}

#[test]
fn test_get_index_valid_end() {
    let mut index_set: super::IndexSet<i32, ()> = super::IndexSet {
        map: super::IndexMap::new(),
    };
    index_set.map.core.push(super::Bucket { hash: 0, key: 1, value: () });
    let _ = index_set.get_index(0);
}

#[test]
fn test_get_index_out_of_bounds() {
    let mut index_set: super::IndexSet<i32, ()> = super::IndexSet {
        map: super::IndexMap::new(),
    };
    index_set.map.core.push(super::Bucket { hash: 0, key: 1, value: () });
    let _ = index_set.get_index(1);
}

#[test]
fn test_get_index_empty() {
    let mut index_set: super::IndexSet<i32, ()> = super::IndexSet {
        map: super::IndexMap::new(),
    };
    let _ = index_set.get_index(0);
}

