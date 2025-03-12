// Answer 0

#[test]
fn test_insert_with_multiple_ranges() {
    let mut range_trie = RangeTrie::new();
    let existing_range1 = Utf8Range { start: 5, end: 10 };
    let existing_range2 = Utf8Range { start: 12, end: 15 };
    range_trie.insert(&[existing_range1, existing_range2]);

    let new_ranges = vec![
        Utf8Range { start: 11, end: 13 },
        Utf8Range { start: 14, end: 16 },
    ];
    
    // This will assert ranges.is_empty() is false, ranges.len() <= 4 is true,
    // and will satisfy all other preconditions including overlapping ranges.
    range_trie.insert(&new_ranges);
}

#[test]
fn test_insert_with_overlapping_ranges() {
    let mut range_trie = RangeTrie::new();
    let existing_range = Utf8Range { start: 0, end: 5 };
    range_trie.insert(&[existing_range]);

    let new_ranges = vec![
        Utf8Range { start: 4, end: 10 }, // This will overlap with the existing range
    ];
    
    // This will assert the necessary preconditions are met for insert,
    // including split ranges that yield more than one partition when overlapping
    range_trie.insert(&new_ranges);
}

#[test]
fn test_insert_with_maximum_length() {
    let mut range_trie = RangeTrie::new();
    let existing_range = Utf8Range { start: 0, end: 2 };
    range_trie.insert(&[existing_range]);

    let new_ranges = vec![
        Utf8Range { start: 3, end: 4 },
        Utf8Range { start: 2, end: 5 },
        Utf8Range { start: 6, end: 7 },
    ];
    
    // This ensures all preconditions are satisfied, specifically checking
    // the handling of multiple ranges up to the maximum length of 4.
    range_trie.insert(&new_ranges);
}

