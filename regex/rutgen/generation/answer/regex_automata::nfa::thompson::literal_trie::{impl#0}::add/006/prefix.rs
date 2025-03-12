// Answer 0

#[test]
fn test_add_literal_forward() {
    let mut trie = LiteralTrie::forward();
    trie.rev = false;
    let input: &[u8] = &[1, 2, 3, 4, 5];
    trie.add(input).unwrap();
}

#[test]
fn test_add_literal_reverse() {
    let mut trie = LiteralTrie::reverse();
    trie.rev = true;
    let input: &[u8] = &[5, 4, 3, 2, 1];
    trie.add(input).unwrap();
}

