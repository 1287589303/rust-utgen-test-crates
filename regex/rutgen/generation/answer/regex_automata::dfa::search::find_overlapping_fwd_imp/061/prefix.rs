// Answer 0

#[test]
fn test_find_overlapping_fwd_imp_case_1() {
    struct TestDFA {
        universal_start: bool,
    }

    impl TestDFA {
        fn new() -> Self {
            TestDFA { universal_start: true }
        }

        fn universal_start_state(&self, _anchored: Anchored) -> Option<StateID> {
            if self.universal_start {
                Some(StateID::default())
            } else {
                None
            }
        }

        fn next_state(&self, _sid: StateID, _byte: u8) -> StateID {
            StateID::default()
        }

        fn is_special_state(&self, _sid: StateID) -> bool {
            true
        }

        fn is_start_state(&self, _sid: StateID) -> bool {
            false
        }

        fn is_match_state(&self, _sid: StateID) -> bool {
            false
        }

        fn is_accel_state(&self, _sid: StateID) -> bool {
            true
        }

        fn match_len(&self, _sid: StateID) -> usize {
            0
        }

        fn match_pattern(&self, _sid: StateID, _match_index: usize) -> PatternID {
            PatternID::default()
        }

        fn accelerator(&self, _sid: StateID) -> &[u8] {
            &[0]
        }
    }

    let dfa = TestDFA::new();
    let haystack: &[u8] = b"test input for finding overlaps";
    let input = Input::new(&haystack)
        .span(Span { start: 0, end: haystack.len() })
        .anchored(Anchored::No);
    let mut state = OverlappingState {
        mat: None,
        id: None,
        at: 0,
        next_match_index: None,
        rev_eoi: false,
    };

    let _result = find_overlapping_fwd_imp(&dfa, &input, None, &mut state);
}

#[test]
fn test_find_overlapping_fwd_imp_case_2() {
    struct TestDFA {
        universal_start: bool,
    }

    impl TestDFA {
        fn new() -> Self {
            TestDFA { universal_start: true }
        }

        fn universal_start_state(&self, _anchored: Anchored) -> Option<StateID> {
            if self.universal_start {
                Some(StateID::default())
            } else {
                None
            }
        }

        fn next_state(&self, _sid: StateID, _byte: u8) -> StateID {
            StateID::default()
        }

        fn is_special_state(&self, _sid: StateID) -> bool {
            true
        }

        fn is_start_state(&self, _sid: StateID) -> bool {
            false
        }

        fn is_match_state(&self, _sid: StateID) -> bool {
            false
        }

        fn is_accel_state(&self, _sid: StateID) -> bool {
            true
        }

        fn match_len(&self, _sid: StateID) -> usize {
            0
        }

        fn match_pattern(&self, _sid: StateID, _match_index: usize) -> PatternID {
            PatternID::default()
        }

        fn accelerator(&self, _sid: StateID) -> &[u8] {
            &[0]
        }
    }

    let dfa = TestDFA::new();
    let haystack: &[u8] = b"boundary start for testing";
    let input = Input::new(&haystack)
        .span(Span { start: 0, end: haystack.len() })
        .anchored(Anchored::No);
    let mut state = OverlappingState {
        mat: None,
        id: None,
        at: input.end(), // testing boundary case where at equals end
        next_match_index: None,
        rev_eoi: false,
    };

    let _result = find_overlapping_fwd_imp(&dfa, &input, None, &mut state);
}

