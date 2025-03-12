// Answer 0

#[test]
fn test_add_transition_valid() {
    let mut trie = RangeTrie::new();
    let from_id = ROOT;
    let next_id = StateID::new_unchecked(2);
    let range = Utf8Range::new(0, 1);
    trie.add_transition(from_id, range, next_id);
}

#[test]
fn test_add_transition_non_overlapping() {
    let mut trie = RangeTrie::new();
    trie.add_empty();
    let from_id = StateID::new_unchecked(1);
    let next_id = StateID::new_unchecked(2);
    let range1 = Utf8Range::new(0, 5);
    let range2 = Utf8Range::new(6, 10);
    trie.add_transition(from_id, range1, next_id);
    trie.add_transition(from_id, range2, next_id);
}

#[test]
fn test_add_transition_lexicographic_order() {
    let mut trie = RangeTrie::new();
    trie.add_empty();
    let from_id = StateID::new_unchecked(1);
    let next_id = StateID::new_unchecked(2);
    let range1 = Utf8Range::new(0, 2);
    let range2 = Utf8Range::new(3, 5);
    trie.add_transition(from_id, range1, next_id);
    trie.add_transition(from_id, range2, next_id);
}

#[test]
#[should_panic]
fn test_add_transition_invalid_next_id() {
    let mut trie = RangeTrie::new();
    trie.add_empty();
    let from_id = StateID::new_unchecked(1);
    let next_id = from_id;
    let range = Utf8Range::new(0, 1);
    trie.add_transition(from_id, range, next_id);
}

#[test]
#[should_panic]
fn test_add_transition_invalid_lexicographic_order() {
    let mut trie = RangeTrie::new();
    trie.add_empty();
    let from_id = StateID::new_unchecked(1);
    let next_id = StateID::new_unchecked(2);
    let range1 = Utf8Range::new(0, 2);
    let range2 = Utf8Range::new(1, 2); // Overlapping range
    trie.add_transition(from_id, range1, next_id);
    trie.add_transition(from_id, range2, next_id);
}

