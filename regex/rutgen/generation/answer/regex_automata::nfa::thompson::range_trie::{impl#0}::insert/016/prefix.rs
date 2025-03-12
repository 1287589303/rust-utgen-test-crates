// Answer 0

#[test]
fn test_insert_with_non_empty_ranges_length_four() {
    let mut trie = RangeTrie::new();
    let ranges: Vec<Utf8Range> = vec![
        Utf8Range { start: 0, end: 5 },
        Utf8Range { start: 6, end: 10 },
        Utf8Range { start: 11, end: 15 },
        Utf8Range { start: 16, end: 20 },
    ];
    trie.insert(&ranges);
}

#[test]
fn test_insert_partitions_and_splits() {
    let mut trie = RangeTrie::new();
    let existing_ranges = vec![
        Utf8Range { start: 1, end: 5 },
        Utf8Range { start: 6, end: 10 },
    ];
    trie.insert(&existing_ranges);
    
    let new_ranges: Vec<Utf8Range> = vec![
        Utf8Range { start: 3, end: 12 },
    ];
    trie.insert(&new_ranges);
}

#[test]
fn test_insert_with_splits_and_both() {
    let mut trie = RangeTrie::new();
    let existing_ranges = vec![
        Utf8Range { start: 5, end: 10 },
    ];
    trie.insert(&existing_ranges);

    let new_ranges: Vec<Utf8Range> = vec![
        Utf8Range { start: 0, end: 4 },
        Utf8Range { start: 11, end: 15 },
    ];
    trie.insert(&new_ranges);
}

