// Answer 0

#[test]
fn test_clear_non_empty_map_with_version() {
    let mut map = Utf8SuffixMap {
        version: 1,
        capacity: 5,
        map: vec![Utf8SuffixEntry {
            version: 1,
            key: Utf8SuffixKey {},
            val: StateID::new(1),
        }],
    };
    map.clear();
}

#[test]
fn test_clear_non_empty_map_with_high_version() {
    let mut map = Utf8SuffixMap {
        version: 10,
        capacity: 5,
        map: vec![Utf8SuffixEntry {
            version: 10,
            key: Utf8SuffixKey {},
            val: StateID::new(2),
        }],
    };
    map.clear();
}

#[test]
fn test_clear_map_with_multiple_entries() {
    let mut map = Utf8SuffixMap {
        version: 5,
        capacity: 3,
        map: vec![
            Utf8SuffixEntry {
                version: 5,
                key: Utf8SuffixKey {},
                val: StateID::new(3),
            },
            Utf8SuffixEntry {
                version: 5,
                key: Utf8SuffixKey {},
                val: StateID::new(4),
            },
        ],
    };
    map.clear();
}

