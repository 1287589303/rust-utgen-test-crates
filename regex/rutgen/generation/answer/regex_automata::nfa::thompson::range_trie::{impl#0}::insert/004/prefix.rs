// Answer 0

#[test]
fn test_insert_with_maximum_ranges() {
    let mut trie = RangeTrie::new();
    let ranges = [
        Utf8Range { start: 10, end: 20 },
        Utf8Range { start: 30, end: 40 },
        Utf8Range { start: 50, end: 60 },
        Utf8Range { start: 70, end: 80 },
    ];
    trie.insert(&ranges);
}

#[test]
fn test_insert_with_three_ranges() {
    let mut trie = RangeTrie::new();
    let ranges = [
        Utf8Range { start: 10, end: 20 },
        Utf8Range { start: 30, end: 40 },
        Utf8Range { start: 50, end: 60 },
    ];
    trie.insert(&ranges);
}

#[test]
fn test_insert_with_two_ranges() {
    let mut trie = RangeTrie::new();
    let ranges = [
        Utf8Range { start: 10, end: 20 },
        Utf8Range { start: 30, end: 40 },
    ];
    trie.insert(&ranges);
}

#[test]
fn test_insert_with_single_range() {
    let mut trie = RangeTrie::new();
    let ranges = [
        Utf8Range { start: 10, end: 20 },
    ];
    trie.insert(&ranges);
}

