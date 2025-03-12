// Answer 0

#[test]
fn test_insert_overlapping_ranges() {
    let mut trie = RangeTrie::new();
    let ranges: [Utf8Range; 4] = [
        Utf8Range { start: 0, end: 50 },
        Utf8Range { start: 25, end: 75 },
        Utf8Range { start: 60, end: 100 },
        Utf8Range { start: 80, end: 255 },
    ];
    
    trie.insert(&ranges);
}

#[test]
fn test_insert_multiple_non_overlapping_ranges() {
    let mut trie = RangeTrie::new();
    let ranges: [Utf8Range; 4] = [
        Utf8Range { start: 0, end: 10 },
        Utf8Range { start: 20, end: 30 },
        Utf8Range { start: 40, end: 50 },
        Utf8Range { start: 60, end: 255 },
    ];
    
    trie.insert(&ranges);
}

#[test]
fn test_insert_partially_overlapping_ranges() {
    let mut trie = RangeTrie::new();
    let ranges: [Utf8Range; 4] = [
        Utf8Range { start: 0, end: 100 },
        Utf8Range { start: 50, end: 150 },
        Utf8Range { start: 120, end: 200 },
        Utf8Range { start: 180, end: 255 },
    ];
    
    trie.insert(&ranges);
}

#[test]
fn test_insert_with_boundary_values() {
    let mut trie = RangeTrie::new();
    let ranges: [Utf8Range; 4] = [
        Utf8Range { start: 0, end: 0 },
        Utf8Range { start: 1, end: 1 },
        Utf8Range { start: 2, end: 255 },
        Utf8Range { start: 10, end: 20 },
    ];
    
    trie.insert(&ranges);
}

#[test]
fn test_insert_with_maximal_ranges() {
    let mut trie = RangeTrie::new();
    let ranges: [Utf8Range; 4] = [
        Utf8Range { start: 0, end: 255 },
        Utf8Range { start: 100, end: 200 },
        Utf8Range { start: 150, end: 250 },
        Utf8Range { start: 200, end: 255 },
    ];
    
    trie.insert(&ranges);
}

