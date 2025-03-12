// Answer 0

#[test]
fn test_from_dense_dfa_with_empty_starts() {
    struct MockDFA {
        patterns: usize,
    }

    impl MockDFA {
        fn starts_for_each_pattern(&self) -> bool {
            true
        }

        fn pattern_len(&self) -> usize {
            self.patterns
        }

        fn starts(&self) -> impl Iterator<Item = (StateID, bool, Start)> {
            std::iter::empty()
        }

        fn to_index(&self, _id: StateID) -> usize {
            0
        }
    }

    let dfa = MockDFA { patterns: 1 };
    let remap: Vec<StateID> = vec![StateID(0)];

    let result = StartTable::from_dense_dfa(&dfa, &remap);
}

#[test]
fn test_from_dense_dfa_with_multiple_patterns_empty_starts() {
    struct MockDFA {
        patterns: usize,
    }

    impl MockDFA {
        fn starts_for_each_pattern(&self) -> bool {
            true
        }

        fn pattern_len(&self) -> usize {
            self.patterns
        }

        fn starts(&self) -> impl Iterator<Item = (StateID, bool, Start)> {
            std::iter::empty()
        }

        fn to_index(&self, _id: StateID) -> usize {
            0
        }
    }

    let dfa = MockDFA { patterns: 3 };
    let remap: Vec<StateID> = vec![StateID(0), StateID(1)];

    let result = StartTable::from_dense_dfa(&dfa, &remap);
}

