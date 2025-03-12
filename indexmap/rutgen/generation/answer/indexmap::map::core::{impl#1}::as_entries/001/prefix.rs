// Answer 0

#[test]
fn test_as_entries_empty() {
    let map: IndexMapCore<usize, usize> = IndexMapCore {
        indices: hash_table::HashTable::new(),
        entries: Vec::new(),
    };
    let _ = map.as_entries();
}

#[test]
fn test_as_entries_one_element() {
    let bucket = Bucket {
        hash: HashValue::default(),
        key: 1,
        value: 10,
    };
    let map: IndexMapCore<usize, usize> = IndexMapCore {
        indices: hash_table::HashTable::new(),
        entries: vec![bucket],
    };
    let _ = map.as_entries();
}

#[test]
fn test_as_entries_multiple_elements() {
    let buckets = vec![
        Bucket { hash: HashValue::default(), key: 1, value: 10 },
        Bucket { hash: HashValue::default(), key: 2, value: 20 },
        Bucket { hash: HashValue::default(), key: 3, value: 30 },
    ];
    let map: IndexMapCore<usize, usize> = IndexMapCore {
        indices: hash_table::HashTable::new(),
        entries: buckets,
    };
    let _ = map.as_entries();
}

#[test]
fn test_as_entries_max_size() {
    let mut buckets = Vec::with_capacity(usize::MAX); // or an appropriate large number
    for i in 0..(usize::MAX / std::mem::size_of::<Bucket<usize, usize>>()) {
        buckets.push(Bucket {
            hash: HashValue::default(),
            key: i,
            value: i as usize * 10,
        });
    }
    let map: IndexMapCore<usize, usize> = IndexMapCore {
        indices: hash_table::HashTable::new(),
        entries: buckets,
    };
    let _ = map.as_entries();
}

