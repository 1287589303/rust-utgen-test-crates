// Answer 0

#[test]
fn test_into_iter_empty_indexmap() {
    struct DummyHasher;

    let index_map: super::IndexMap<i32, i32, DummyHasher> = super::IndexMap {
        core: super::IndexMapCore {
            indices: super::Indices::new(),
            entries: super::Entries::new(),
        },
        hash_builder: DummyHasher,
    };

    let iter = index_map.into_iter();
}

#[test]
fn test_into_iter_single_entry_indexmap() {
    struct DummyHasher;

    let mut index_map: super::IndexMap<i32, String, DummyHasher> = super::IndexMap {
        core: super::IndexMapCore {
            indices: super::Indices::new(),
            entries: super::Entries::new(),
        },
        hash_builder: DummyHasher,
    };

    index_map.core.entries.push(super::Bucket { hash: 1, key: 1, value: String::from("one") });

    let iter = index_map.into_iter();
}

#[test]
fn test_into_iter_multiple_entries_indexmap() {
    struct DummyHasher;

    let mut index_map: super::IndexMap<String, Vec<i32>, DummyHasher> = super::IndexMap {
        core: super::IndexMapCore {
            indices: super::Indices::new(),
            entries: super::Entries::new(),
        },
        hash_builder: DummyHasher,
    };

    index_map.core.entries.push(super::Bucket { hash: 1, key: String::from("one"), value: vec![1, 2, 3] });
    index_map.core.entries.push(super::Bucket { hash: 2, key: String::from("two"), value: vec![4, 5, 6] });

    let iter = index_map.into_iter();
}

#[test]
fn test_into_iter_max_size_indexmap() {
    struct DummyHasher;

    let mut index_map: super::IndexMap<u64, u64, DummyHasher> = super::IndexMap {
        core: super::IndexMapCore {
            indices: super::Indices::new(),
            entries: super::Entries::new(),
        },
        hash_builder: DummyHasher,
    };

    for i in 0..1000 {
        index_map.core.entries.push(super::Bucket { hash: i as u64, key: i, value: i });
    }

    let iter = index_map.into_iter();
}

