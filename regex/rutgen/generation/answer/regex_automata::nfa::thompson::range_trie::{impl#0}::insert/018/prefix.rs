// Answer 0

#[test]
fn test_insert_with_full_range() {
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
fn test_insert_with_adjacency() {
    let mut trie = RangeTrie::new();
    let ranges = [
        Utf8Range { start: 0, end: 0 },
        Utf8Range { start: 1, end: 1 },
        Utf8Range { start: 2, end: 2 },
        Utf8Range { start: 3, end: 3 },
    ];
    trie.insert(&ranges);
}

#[test]
fn test_insert_with_overlap() {
    let mut trie = RangeTrie::new();
    let ranges = [
        Utf8Range { start: 0, end: 2 },
        Utf8Range { start: 1, end: 3 },
        Utf8Range { start: 4, end: 6 },
        Utf8Range { start: 5, end: 7 },
    ];
    trie.insert(&ranges);
}

#[test]
fn test_insert_with_strict_next() {
    let mut trie = RangeTrie::new();
    let ranges = [
        Utf8Range { start: 0, end: 2 },
        Utf8Range { start: 3, end: 5 },
        Utf8Range { start: 6, end: 8 },
        Utf8Range { start: 9, end: 11 },
    ];
    trie.insert(&ranges);
}

