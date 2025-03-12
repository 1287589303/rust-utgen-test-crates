// Answer 0

#[test]
fn test_insert_single_range() {
    let mut trie = RangeTrie::new();
    let range = Utf8Range { start: 1, end: 2 };
    trie.insert(&[range]);
}

#[test]
fn test_insert_multiple_ranges() {
    let mut trie = RangeTrie::new();
    let ranges = [
        Utf8Range { start: 1, end: 2 },
        Utf8Range { start: 3, end: 4 },
    ];
    trie.insert(&ranges);
}

#[test]
fn test_insert_four_ranges() {
    let mut trie = RangeTrie::new();
    let ranges = [
        Utf8Range { start: 0, end: 1 },
        Utf8Range { start: 2, end: 3 },
        Utf8Range { start: 4, end: 5 },
        Utf8Range { start: 6, end: 7 },
    ];
    trie.insert(&ranges);
}

#[should_panic]
#[test]
fn test_insert_empty_ranges() {
    let mut trie = RangeTrie::new();
    trie.insert(&[]); // will panic due to empty range
}

#[should_panic]
#[test]
fn test_insert_too_many_ranges() {
    let mut trie = RangeTrie::new();
    let ranges = [
        Utf8Range { start: 0, end: 1 },
        Utf8Range { start: 2, end: 3 },
        Utf8Range { start: 4, end: 5 },
        Utf8Range { start: 6, end: 7 },
        Utf8Range { start: 8, end: 9 },
    ];
    trie.insert(&ranges); // will panic due to too many ranges
}

#[test]
fn test_insert_overlapping_ranges() {
    let mut trie = RangeTrie::new();
    let ranges = [
        Utf8Range { start: 1, end: 3 },
        Utf8Range { start: 2, end: 4 },
    ];
    trie.insert(&ranges);
}

