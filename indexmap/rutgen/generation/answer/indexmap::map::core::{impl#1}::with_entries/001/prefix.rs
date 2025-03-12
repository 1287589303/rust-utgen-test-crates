// Answer 0

#[test]
fn test_with_entries_empty() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.with_entries(|entries| {
        assert_eq!(entries.len(), 0);
        // Perform an operation, e.g., extending entries
        entries.push(Bucket {
            hash: HashValue::new(1),
            key: 1,
            value: 10,
        });
    });
}

#[test]
fn test_with_entries_one() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.with_entries(|entries| {
        assert_eq!(entries.len(), 0);
        entries.push(Bucket {
            hash: HashValue::new(1),
            key: 1,
            value: 10,
        });
    });
    assert_eq!(map.as_entries().len(), 1);
}

#[test]
fn test_with_entries_max_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(IndexMapCore::MAX_ENTRIES_CAPACITY);
    map.with_entries(|entries| {
        assert_eq!(entries.len(), 0);
        for i in 0..IndexMapCore::MAX_ENTRIES_CAPACITY {
            entries.push(Bucket {
                hash: HashValue::new(i as u64),
                key: i,
                value: i * 10,
            });
        }
    });
    assert_eq!(map.as_entries().len(), IndexMapCore::MAX_ENTRIES_CAPACITY);
}

#[test]
fn test_with_entries_near_max_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(IndexMapCore::MAX_ENTRIES_CAPACITY);
    map.with_entries(|entries| {
        assert_eq!(entries.len(), 0);
        for i in 0..(IndexMapCore::MAX_ENTRIES_CAPACITY - 1) {
            entries.push(Bucket {
                hash: HashValue::new(i as u64),
                key: i,
                value: i * 10,
            });
        }
    });
    assert_eq!(map.as_entries().len(), IndexMapCore::MAX_ENTRIES_CAPACITY - 1);
}

