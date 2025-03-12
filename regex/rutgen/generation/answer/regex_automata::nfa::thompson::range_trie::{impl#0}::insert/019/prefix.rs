// Answer 0

#[test]
fn test_insert_with_non_overlapping_ranges() {
    let mut trie = RangeTrie::new();
    let transition = Transition {
        range: Utf8Range { start: 1, end: 3 },
        next_id: StateID::new_unchecked(2),
    };
    trie.states.push(State {
        transitions: vec![transition],
    });

    let ranges = vec![
        Utf8Range { start: 4, end: 5 },
    ];
    trie.insert(&ranges);
}

#[test]
fn test_insert_with_multiple_ranges() {
    let mut trie = RangeTrie::new();
    let transition1 = Transition {
        range: Utf8Range { start: 1, end: 3 },
        next_id: StateID::new_unchecked(2),
    };
    let transition2 = Transition {
        range: Utf8Range { start: 6, end: 8 },
        next_id: StateID::new_unchecked(3),
    };
    trie.states.push(State {
        transitions: vec![transition1, transition2],
    });

    let ranges = vec![
        Utf8Range { start: 9, end: 10 },
        Utf8Range { start: 11, end: 12 },
    ];
    trie.insert(&ranges);
}

#[test]
fn test_insert_split_with_single_partition() {
    let mut trie = RangeTrie::new();
    let transition = Transition {
        range: Utf8Range { start: 1, end: 5 },
        next_id: StateID::new_unchecked(2),
    };
    trie.states.push(State {
        transitions: vec![transition],
    });

    let ranges = vec![
        Utf8Range { start: 2, end: 4 },
    ];
    trie.insert(&ranges);
}

#[test]
fn test_insert_split_with_muliple_partitions() {
    let mut trie = RangeTrie::new();
    let transition = Transition {
        range: Utf8Range { start: 1, end: 5 },
        next_id: StateID::new_unchecked(2),
    };
    trie.states.push(State {
        transitions: vec![transition],
    });

    let ranges = vec![
        Utf8Range { start: 3, end: 6 },
        Utf8Range { start: 7, end: 8 },
    ];
    trie.insert(&ranges);
}

