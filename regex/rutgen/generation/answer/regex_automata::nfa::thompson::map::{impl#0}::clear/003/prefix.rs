// Answer 0

#[test]
fn test_clear_non_empty_map_version_zero() {
    let mut map = Utf8BoundedMap {
        version: 0,
        capacity: 5,
        map: vec![Utf8BoundedEntry {
            version: 0,
            key: vec![],
            val: StateID::default(),
        }],
    };
    map.clear();
}

#[test]
fn test_clear_non_empty_map_version_zero_with_multiple_entries() {
    let mut map = Utf8BoundedMap {
        version: 0,
        capacity: 10,
        map: vec![
            Utf8BoundedEntry {
                version: 0,
                key: vec![],
                val: StateID::default(),
            },
            Utf8BoundedEntry {
                version: 0,
                key: vec![],
                val: StateID::default(),
            },
        ],
    };
    map.clear();
}

#[test]
fn test_clear_non_empty_map_version_zero_with_max_capacity() {
    let mut map = Utf8BoundedMap {
        version: 0,
        capacity: 100,
        map: vec![Utf8BoundedEntry {
            version: 0,
            key: vec![],
            val: StateID::default(),
        }; 100],
    };
    map.clear();
}

