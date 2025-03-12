// Answer 0

#[test]
fn test_find_overlapping_rev_case_1() {
    struct DummyAutomaton;
    
    impl Automaton for DummyAutomaton {
        fn start_state_reverse(&self, input: &Input<'_>) -> Result<StateID, MatchError> {
            Ok(StateID::default())
        }
        
        fn is_special_state(&self, _sid: StateID) -> bool { true }

        fn is_start_state(&self, _sid: StateID) -> bool { true }

        fn is_accel_state(&self, _sid: StateID) -> bool { false }

        fn next_state(&self, sid: StateID, _byte: u8) -> StateID {
            sid
        }

        fn match_pattern(&self, _sid: StateID, _index: usize) -> PatternID {
            PatternID::default()
        }

        fn match_len(&self, _sid: StateID) -> usize {
            1
        }
    }

    let haystack = b"test haystack";
    let input = Input::new(&haystack).span(Span { start: 0, end: haystack.len() }).anchored(Anchored::No);
    let mut state = OverlappingState {
        mat: Some(HalfMatch::default()),
        id: None,
        at: input.start(),
        next_match_index: None,
        rev_eoi: false,
    };
    let dfa = DummyAutomaton;

    let result = find_overlapping_rev(&dfa, &input, &mut state);
}

#[test]
fn test_find_overlapping_rev_case_2() {
    struct DummyAutomaton;
    
    impl Automaton for DummyAutomaton {
        fn start_state_reverse(&self, input: &Input<'_>) -> Result<StateID, MatchError> {
            Ok(StateID::default())
        }
        
        fn is_special_state(&self, _sid: StateID) -> bool { true }

        fn is_start_state(&self, _sid: StateID) -> bool { true }

        fn is_accel_state(&self, _sid: StateID) -> bool { false }

        fn next_state(&self, sid: StateID, _byte: u8) -> StateID {
            sid
        }

        fn match_pattern(&self, _sid: StateID, _index: usize) -> PatternID {
            PatternID::default()
        }

        fn match_len(&self, _sid: StateID) -> usize {
            2
        }
    }

    let haystack = b"another test haystack";
    let input = Input::new(&haystack).span(Span { start: 0, end: haystack.len() }).anchored(Anchored::No);
    let mut state = OverlappingState {
        mat: Some(HalfMatch::default()),
        id: None,
        at: input.start(),
        next_match_index: None,
        rev_eoi: false,
    };
    let dfa = DummyAutomaton;

    let result = find_overlapping_rev(&dfa, &input, &mut state);
}

