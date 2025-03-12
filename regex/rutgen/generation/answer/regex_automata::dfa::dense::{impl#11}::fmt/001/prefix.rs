// Answer 0

#[test]
fn test_fmt_dfa_err() {
    struct MockState;
    impl MockState {
        fn id(&self) -> StateID {
            StateID(0)
        }
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    struct MockDFA {
        states: Vec<MockState>,
        match_states: MatchStates<Vec<u32>>,
        start_table: StartTable<Vec<u32>>,
    }

    impl MockDFA {
        fn states(&self) -> impl Iterator<Item = &MockState> {
            self.states.iter()
        }
        
        fn starts(&self) -> impl Iterator<Item = (StateID, Anchored, &str)> {
            self.start_table.table.iter().enumerate().map(|(i, _)| {
                (StateID(i as u32), Anchored::No, "")
            })
        }
        
        fn pattern_len(&self) -> usize {
            1
        }

        fn state_len(&self) -> usize {
            self.states.len()
        }

        fn ms(&self) -> &MatchStates<Vec<u32>> {
            &self.match_states
        }
    }

    let formatter = &mut fmt::Formatter::new();
    let dfa = MockDFA {
        states: vec![MockState],
        match_states: MatchStates {
            slices: vec![0],
            pattern_ids: vec![0],
            pattern_len: 1,
        },
        start_table: StartTable {
            table: vec![0],
            kind: StartKind::Both,
            start_map: StartByteMap {},
            stride: 1,
            pattern_len: Some(1),
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
    };

    let _ = dfa.fmt(formatter);
}

#[test]
fn test_fmt_dfa_start_err() {
    struct MockState;
    impl MockState {
        fn id(&self) -> StateID {
            StateID(1)
        }
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            Ok(())
        }
    }

    struct MockDFA {
        states: Vec<MockState>,
        match_states: MatchStates<Vec<u32>>,
        start_table: StartTable<Vec<u32>>,
    }

    impl MockDFA {
        fn states(&self) -> impl Iterator<Item = &MockState> {
            self.states.iter()
        }
        
        fn starts(&self) -> impl Iterator<Item = (StateID, Anchored, &str)> {
            vec![(StateID(0), Anchored::No, ""), (StateID(1), Anchored::Yes, "")]
                .into_iter()
        }
        
        fn pattern_len(&self) -> usize {
            1
        }

        fn state_len(&self) -> usize {
            self.states.len()
        }

        fn ms(&self) -> &MatchStates<Vec<u32>> {
            &self.match_states
        }
    }

    let formatter = &mut fmt::Formatter::new();
    let dfa = MockDFA {
        states: vec![MockState, MockState],
        match_states: MatchStates {
            slices: vec![0],
            pattern_ids: vec![0],
            pattern_len: 1,
        },
        start_table: StartTable {
            table: vec![0, 1],
            kind: StartKind::Both,
            start_map: StartByteMap {},
            stride: 1,
            pattern_len: Some(2),
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
    };

    let _ = dfa.fmt(formatter);
}

