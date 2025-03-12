// Answer 0

#[test]
fn test_insert_range_length_1() {
    let mut trie = RangeTrie::new();
    let ranges = [Utf8Range { start: 0, end: 1 }];
    trie.insert(&ranges);
}

#[test]
fn test_insert_range_length_2() {
    let mut trie = RangeTrie::new();
    let ranges = [
        Utf8Range { start: 0, end: 1 },
        Utf8Range { start: 2, end: 3 },
    ];
    trie.insert(&ranges);
}

#[test]
fn test_insert_range_length_3() {
    let mut trie = RangeTrie::new();
    let ranges = [
        Utf8Range { start: 0, end: 1 },
        Utf8Range { start: 2, end: 3 },
        Utf8Range { start: 4, end: 5 },
    ];
    trie.insert(&ranges);
}

#[test]
fn test_insert_range_length_4() {
    let mut trie = RangeTrie::new();
    let ranges = [
        Utf8Range { start: 0, end: 1 },
        Utf8Range { start: 2, end: 3 },
        Utf8Range { start: 4, end: 5 },
        Utf8Range { start: 6, end: 7 },
    ];
    trie.insert(&ranges);
}

