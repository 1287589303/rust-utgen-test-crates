// Answer 0

#[test]
fn test_with_entries_empty() {
    let mut index_set: super::IndexSet<i32, ()> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: (),
        },
    };
    index_set.with_entries(|entries| {
        // No operation, since there are no entries
    });
}

#[test]
fn test_with_entries_single() {
    let mut index_set: super::IndexSet<i32, ()> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::from_entries(vec![super::Bucket {
                hash: HashValue::default(),
                key: 1,
                value: (),
            }]),
            hash_builder: (),
        },
    };
    index_set.with_entries(|entries| {
        if let Some(entry) = entries.get_mut(0) {
            entry.value = (); // Modify the entry value
        }
    });
}

#[test]
fn test_with_entries_multiple() {
    let mut index_set: super::IndexSet<i32, ()> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::from_entries(vec![
                super::Bucket {
                    hash: HashValue::default(),
                    key: 1,
                    value: (),
                },
                super::Bucket {
                    hash: HashValue::default(),
                    key: 2,
                    value: (),
                },
            ]),
            hash_builder: (),
        },
    };
    index_set.with_entries(|entries| {
        for entry in entries.iter_mut() {
            entry.value = (); // Modify each entry's value
        }
    });
}

#[test]
fn test_with_entries_filtering() {
    let mut index_set: super::IndexSet<i32, ()> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::from_entries(vec![
                super::Bucket {
                    hash: HashValue::default(),
                    key: 1,
                    value: (),
                },
                super::Bucket {
                    hash: HashValue::default(),
                    key: 2,
                    value: (),
                },
                super::Bucket {
                    hash: HashValue::default(),
                    key: 3,
                    value: (),
                },
            ]),
            hash_builder: (),
        },
    };
    index_set.with_entries(|entries| {
        entries.retain(|entry| entry.key != 2); // Filter out entry with key 2
    });
}

