// Answer 0

#[test]
fn test_push_with_empty_ranges() {
    let mut trie = RangeTrie::new();
    let mut stack = vec![];
    let result = NextInsert::push(&mut trie, &mut stack, &[]);
}

#[test]
fn test_push_with_empty_ranges_after_initialization() {
    let mut trie = RangeTrie::new();
    let mut stack = vec![];
    let result = NextInsert::push(&mut trie, &mut stack, &[]);
}

