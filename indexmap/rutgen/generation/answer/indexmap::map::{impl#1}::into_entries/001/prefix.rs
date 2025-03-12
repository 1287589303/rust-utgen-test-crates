// Answer 0

#[test]
fn test_into_entries_empty_map() {
    let map: super::IndexMap<i32, i32, std::collections::hash_map::RandomState> = super::IndexMap {
        core: super::IndexMapCore {
            indices: super::Indices::new(),
            entries: super::Entries::new(),
        },
        hash_builder: std::collections::hash_map::RandomState::new(),
    };
    let entries = map.into_entries();
}

#[test]
fn test_into_entries_single_entry() {
    let mut map: super::IndexMap<i32, i32, std::collections::hash_map::RandomState> = super::IndexMap {
        core: super::IndexMapCore {
            indices: super::Indices::new(),
            entries: super::Entries::from(vec![super::Bucket {
                hash: super::HashValue::new(1),
                key: 1,
                value: 100,
            }]),
        },
        hash_builder: std::collections::hash_map::RandomState::new(),
    };
    let entries = map.into_entries();
}

#[test]
fn test_into_entries_multiple_entries() {
    let entries_vec = vec![
        super::Bucket {
            hash: super::HashValue::new(1),
            key: 1,
            value: 100,
        },
        super::Bucket {
            hash: super::HashValue::new(2),
            key: 2,
            value: 200,
        },
        super::Bucket {
            hash: super::HashValue::new(3),
            key: 3,
            value: 300,
        },
    ];
    
    let mut map: super::IndexMap<i32, i32, std::collections::hash_map::RandomState> = super::IndexMap {
        core: super::IndexMapCore {
            indices: super::Indices::new(),
            entries: super::Entries::from(entries_vec),
        },
        hash_builder: std::collections::hash_map::RandomState::new(),
    };
    let entries = map.into_entries();
}

#[test]
fn test_into_entries_with_duplicate_keys() {
    let entries_vec = vec![
        super::Bucket {
            hash: super::HashValue::new(1),
            key: 1,
            value: 100,
        },
        super::Bucket {
            hash: super::HashValue::new(1),
            key: 1,
            value: 200,
        },
    ];

    let mut map: super::IndexMap<i32, i32, std::collections::hash_map::RandomState> = super::IndexMap {
        core: super::IndexMapCore {
            indices: super::Indices::new(),
            entries: super::Entries::from(entries_vec),
        },
        hash_builder: std::collections::hash_map::RandomState::new(),
    };
    let entries = map.into_entries();
}

