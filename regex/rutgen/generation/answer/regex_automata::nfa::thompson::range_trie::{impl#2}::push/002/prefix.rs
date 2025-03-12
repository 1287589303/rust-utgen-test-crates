// Answer 0

#[test]
fn test_push_with_one_range() {
    let mut trie = RangeTrie::new();
    trie.add_empty(); // adding root state
    let mut stack = vec![];
    let ranges = &[Utf8Range { start: 1, end: 2 }];
    let next_id = NextInsert::push(&mut trie, &mut stack, ranges);
}

#[test]
fn test_push_with_two_ranges() {
    let mut trie = RangeTrie::new();
    trie.add_empty(); // adding root state
    let mut stack = vec![];
    let ranges = &[
        Utf8Range { start: 1, end: 2 },
        Utf8Range { start: 3, end: 4 },
    ];
    let next_id = NextInsert::push(&mut trie, &mut stack, ranges);
}

#[test]
fn test_push_with_three_ranges() {
    let mut trie = RangeTrie::new();
    trie.add_empty(); // adding root state
    let mut stack = vec![];
    let ranges = &[
        Utf8Range { start: 1, end: 2 },
        Utf8Range { start: 3, end: 4 },
        Utf8Range { start: 5, end: 6 },
    ];
    let next_id = NextInsert::push(&mut trie, &mut stack, ranges);
}

#[test]
fn test_push_with_four_ranges() {
    let mut trie = RangeTrie::new();
    trie.add_empty(); // adding root state
    let mut stack = vec![];
    let ranges = &[
        Utf8Range { start: 1, end: 2 },
        Utf8Range { start: 3, end: 4 },
        Utf8Range { start: 5, end: 6 },
        Utf8Range { start: 7, end: 8 },
    ];
    let next_id = NextInsert::push(&mut trie, &mut stack, ranges);
}

