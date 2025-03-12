// Answer 0

#[test]
fn test_insert_with_adjacent_ranges() {
    let mut trie = RangeTrie::new();
    let ranges = [
        Utf8Range { start: 0, end: 1 },
        Utf8Range { start: 2, end: 3 },
        Utf8Range { start: 4, end: 5 },
        Utf8Range { start: 6, end: 7 },
    ];
    trie.insert(&ranges);
}

#[test]
fn test_insert_with_overlapping_ranges() {
    let mut trie = RangeTrie::new();
    let ranges = [
        Utf8Range { start: 1, end: 3 },
        Utf8Range { start: 3, end: 5 },
        Utf8Range { start: 5, end: 7 },
        Utf8Range { start: 7, end: 9 },
    ];
    trie.insert(&ranges);
}

#[test]
fn test_insert_with_large_ranges() {
    let mut trie = RangeTrie::new();
    let ranges = [
        Utf8Range { start: 10, end: 20 },
        Utf8Range { start: 21, end: 30 },
        Utf8Range { start: 31, end: 40 },
        Utf8Range { start: 41, end: 50 },
    ];
    trie.insert(&ranges);
}

