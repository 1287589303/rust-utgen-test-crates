// Answer 0

#[test]
fn test_clear_non_empty_map_non_zero_version() {
    let mut map = Utf8BoundedMap {
        version: 1,
        capacity: 1,
        map: vec![Utf8BoundedEntry {
            version: 1,
            key: vec![],
            val: StateID(0),
        }],
    };
    map.clear();
}

#[test]
fn test_clear_non_empty_map_high_version() {
    let mut map = Utf8BoundedMap {
        version: 65535,
        capacity: 2,
        map: vec![
            Utf8BoundedEntry {
                version: 65535,
                key: vec![],
                val: StateID(1),
            },
            Utf8BoundedEntry {
                version: 65535,
                key: vec![],
                val: StateID(2),
            },
        ],
    };
    map.clear();
}

#[test]
fn test_clear_non_empty_map_with_multiple_entries() {
    let mut map = Utf8BoundedMap {
        version: 100,
        capacity: 3,
        map: vec![
            Utf8BoundedEntry {
                version: 100,
                key: vec![],
                val: StateID(3),
            },
            Utf8BoundedEntry {
                version: 100,
                key: vec![],
                val: StateID(4),
            },
            Utf8BoundedEntry {
                version: 100,
                key: vec![],
                val: StateID(5),
            },
        ],
    };
    map.clear();
}

