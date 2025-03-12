// Answer 0

#[test]
#[should_panic]
fn test_insert_empty_ranges() {
    let mut trie = RangeTrie::new();
    trie.insert(&[]);
}

#[test]
fn test_insert_single_range() {
    let mut trie = RangeTrie::new();
    let ranges = [Utf8Range { start: 1, end: 2 }];
    trie.insert(&ranges);
}

#[test]
fn test_insert_two_ranges() {
    let mut trie = RangeTrie::new();
    let ranges = [
        Utf8Range { start: 1, end: 2 },
        Utf8Range { start: 3, end: 4 },
    ];
    trie.insert(&ranges);
}

#[test]
fn test_insert_three_ranges() {
    let mut trie = RangeTrie::new();
    let ranges = [
        Utf8Range { start: 1, end: 2 },
        Utf8Range { start: 3, end: 4 },
        Utf8Range { start: 5, end: 6 },
    ];
    trie.insert(&ranges);
}

#[test]
fn test_insert_four_ranges() {
    let mut trie = RangeTrie::new();
    let ranges = [
        Utf8Range { start: 1, end: 2 },
        Utf8Range { start: 3, end: 4 },
        Utf8Range { start: 5, end: 6 },
        Utf8Range { start: 7, end: 8 },
    ];
    trie.insert(&ranges);
}

