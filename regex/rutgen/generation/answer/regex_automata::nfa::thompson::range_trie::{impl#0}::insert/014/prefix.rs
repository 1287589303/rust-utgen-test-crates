// Answer 0

#[test]
fn test_insert_with_multiple_ranges_overlapping() {
    let mut trie = RangeTrie::new();
    let ranges: &[Utf8Range] = &[
        Utf8Range { start: 5, end: 10 },
        Utf8Range { start: 20, end: 25 },
        Utf8Range { start: 15, end: 22 },
        Utf8Range { start: 30, end: 35 },
    ];
    
    trie.insert(ranges);
}

#[test]
fn test_insert_with_single_matching_range() {
    let mut trie = RangeTrie::new();
    let existing_ranges: &[Utf8Range] = &[Utf8Range { start: 1, end: 3 }];
    trie.insert(existing_ranges);

    let new_ranges: &[Utf8Range] = &[Utf8Range { start: 2, end: 5 }];
    trie.insert(new_ranges);
}

#[test]
fn test_insert_with_two_ranges_that_intersect() {
    let mut trie = RangeTrie::new();
    let existing_ranges: &[Utf8Range] = &[
        Utf8Range { start: 1, end: 5 },
        Utf8Range { start: 6, end: 10 },
    ];
    trie.insert(existing_ranges);

    let new_ranges: &[Utf8Range] = &[Utf8Range { start: 4, end: 7 }];
    trie.insert(new_ranges);
}

#[test]
fn test_insert_with_four_ranges_and_complex_overlaps() {
    let mut trie = RangeTrie::new();
    let existing_ranges: &[Utf8Range] = &[
        Utf8Range { start: 0, end: 2 },
        Utf8Range { start: 3, end: 5 },
        Utf8Range { start: 10, end: 15 },
    ];
    trie.insert(existing_ranges);

    let new_ranges: &[Utf8Range] = &[
        Utf8Range { start: 1, end: 4 },
        Utf8Range { start: 5, end: 6 },
        Utf8Range { start: 14, end: 20 },
        Utf8Range { start: 15, end: 18 },
    ];
    trie.insert(new_ranges);
}

