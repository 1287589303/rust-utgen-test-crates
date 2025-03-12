// Answer 0

#[test]
fn test_iter_mut2_non_empty() {
    struct HashBuilder;
    
    let mut index_map: IndexMap<i32, i32, HashBuilder> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: HashBuilder,
    };
    
    index_map.core.entries.push(Bucket { hash: 1, key: 1, value: 100 });
    index_map.core.entries.push(Bucket { hash: 2, key: 2, value: 200 });
    
    let mut iter = index_map.iter_mut2();
    let slice = iter.as_slice();
}

#[test]
fn test_iter_mut2_single_entry() {
    struct HashBuilder;

    let mut index_map: IndexMap<i32, i32, HashBuilder> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: HashBuilder,
    };

    index_map.core.entries.push(Bucket { hash: 3, key: 3, value: 300 });

    let mut iter = index_map.iter_mut2();
    let slice = iter.as_slice();
}

#[test]
fn test_iter_mut2_multiple_entries() {
    struct HashBuilder;

    let mut index_map: IndexMap<i32, i32, HashBuilder> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: HashBuilder,
    };

    index_map.core.entries.push(Bucket { hash: 4, key: 4, value: 400 });
    index_map.core.entries.push(Bucket { hash: 5, key: 5, value: 500 });
    index_map.core.entries.push(Bucket { hash: 6, key: 6, value: 600 });

    let mut iter = index_map.iter_mut2();
    let slice = iter.as_slice();
}

#[test]
#[should_panic]
fn test_iter_mut2_empty_entries() {
    struct HashBuilder;

    let mut index_map: IndexMap<i32, i32, HashBuilder> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: HashBuilder,
    };

    let mut iter = index_map.iter_mut2();
    let slice = iter.as_slice(); // This should panic if entries are empty
}

