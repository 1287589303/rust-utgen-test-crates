// Answer 0

#[test]
fn test_insert_with_max_length_ranges() {
    let mut trie = RangeTrie::new();
    let ranges: &[Utf8Range] = &[
        Utf8Range { start: 0, end: 1 },
        Utf8Range { start: 2, end: 3 },
        Utf8Range { start: 4, end: 5 },
        Utf8Range { start: 6, end: 7 },
    ];
    trie.insert(ranges);
}

#[test]
fn test_insert_with_overlapping_ranges() {
    let mut trie = RangeTrie::new();
    trie.insert(&[Utf8Range { start: 1, end: 3 }]);
    let ranges: &[Utf8Range] = &[
        Utf8Range { start: 2, end: 4 },
        Utf8Range { start: 5, end: 5 },
    ];
    trie.insert(ranges);
}

#[test]
fn test_insert_with_partially_overlapping_ranges() {
    let mut trie = RangeTrie::new();
    trie.insert(&[Utf8Range { start: 1, end: 5 }]);
    let ranges: &[Utf8Range] = &[
        Utf8Range { start: 3, end: 4 },
        Utf8Range { start: 6, end: 8 },
    ];
    trie.insert(ranges);
}

#[test]
fn test_insert_with_no_overlap() {
    let mut trie = RangeTrie::new();
    trie.insert(&[Utf8Range { start: 0, end: 0 }]);
    let ranges: &[Utf8Range] = &[
        Utf8Range { start: 2, end: 2 },
        Utf8Range { start: 3, end: 3 },
    ];
    trie.insert(ranges);
} 

#[test]
fn test_insert_with_some_overlap() {
    let mut trie = RangeTrie::new();
    trie.insert(&[Utf8Range { start: 0, end: 5 }]);
    let ranges: &[Utf8Range] = &[
        Utf8Range { start: 5, end: 7 },
        Utf8Range { start: 8, end: 10 },
    ];
    trie.insert(ranges);
}

