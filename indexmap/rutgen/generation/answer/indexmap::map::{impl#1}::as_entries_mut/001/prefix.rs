// Answer 0

#[test]
fn test_as_entries_mut_non_empty() {
    let mut index_map: crate::IndexMap<i32, String, ::std::collections::hash_map::RandomState> = crate::IndexMap {
        core: crate::IndexMapCore {
            indices: crate::Indices::new(),
            entries: crate::Entries::from(vec![
                crate::Bucket { hash: crate::HashValue::default(), key: 1, value: "one".to_string() },
                crate::Bucket { hash: crate::HashValue::default(), key: 2, value: "two".to_string() },
            ]),
        },
        hash_builder: ::std::collections::hash_map::RandomState::new(),
    };
    let _entries_mut = index_map.as_entries_mut();
}

#[test]
fn test_as_entries_mut_empty() {
    let mut index_map: crate::IndexMap<i32, String, ::std::collections::hash_map::RandomState> = crate::IndexMap {
        core: crate::IndexMapCore {
            indices: crate::Indices::new(),
            entries: crate::Entries::from(vec![]),
        },
        hash_builder: ::std::collections::hash_map::RandomState::new(),
    };
    let entries_mut = index_map.as_entries_mut();
    assert!(entries_mut.is_empty());
}

#[test]
fn test_as_entries_mut_single_entry() {
    let mut index_map: crate::IndexMap<i32, String, ::std::collections::hash_map::RandomState> = crate::IndexMap {
        core: crate::IndexMapCore {
            indices: crate::Indices::new(),
            entries: crate::Entries::from(vec![crate::Bucket { hash: crate::HashValue::default(), key: 1, value: "one".to_string() }]),
        },
        hash_builder: ::std::collections::hash_map::RandomState::new(),
    };
    let entries_mut = index_map.as_entries_mut();
    assert_eq!(entries_mut.len(), 1);
}

#[test]
fn test_as_entries_mut_multiple_entries() {
    let mut index_map: crate::IndexMap<i32, String, ::std::collections::hash_map::RandomState> = crate::IndexMap {
        core: crate::IndexMapCore {
            indices: crate::Indices::new(),
            entries: crate::Entries::from(vec![
                crate::Bucket { hash: crate::HashValue::default(), key: 1, value: "one".to_string() },
                crate::Bucket { hash: crate::HashValue::default(), key: 2, value: "two".to_string() },
                crate::Bucket { hash: crate::HashValue::default(), key: 3, value: "three".to_string() },
            ]),
        },
        hash_builder: ::std::collections::hash_map::RandomState::new(),
    };
    let entries_mut = index_map.as_entries_mut();
    assert_eq!(entries_mut.len(), 3);
}

