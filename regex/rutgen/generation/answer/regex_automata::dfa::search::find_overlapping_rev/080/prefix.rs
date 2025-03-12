// Answer 0

#[test]
fn test_find_overlapping_rev_case_1() {
    #[derive(Clone)]
    struct MockDFA;

    impl MockDFA {
        fn start_state_reverse(&self, _input: &Input) -> Result<StateID, MatchError> {
            Ok(StateID::default())
        }
        fn is_special_state(&self, _sid: StateID) -> bool { true }
        fn is_start_state(&self, _sid: StateID) -> bool { false }
        fn is_match_state(&self, _sid: StateID) -> bool { false }
        fn is_accel_state(&self, _sid: StateID) -> bool { true }
        fn match_len(&self, _sid: StateID) -> usize { 1 }
        fn match_pattern(&self, _sid: StateID, _index: usize) -> PatternID { PatternID::default() }
        fn next_state(&self, _sid: StateID, _byte: u8) -> StateID { StateID::default() }
        fn accelerator(&self, _sid: StateID) -> &[u8] { &[1] }
    }

    let haystack = b"hello world";
    let input = Input::new(&haystack).set_span(0..10);
    let mut state = OverlappingState {
        mat: None,
        id: Some(StateID::default()),
        at: 9, // greater than input.start()
        next_match_index: Some(1), // equals match_len
        rev_eoi: false,
    };

    let dfa = MockDFA;
    let _result = find_overlapping_rev(&dfa, &input, &mut state);
}

#[test]
fn test_find_overlapping_rev_case_2() {
    #[derive(Clone)]
    struct MockDFA;

    impl MockDFA {
        fn start_state_reverse(&self, _input: &Input) -> Result<StateID, MatchError> {
            Ok(StateID::default())
        }
        fn is_special_state(&self, _sid: StateID) -> bool { true }
        fn is_start_state(&self, _sid: StateID) -> bool { false }
        fn is_match_state(&self, _sid: StateID) -> bool { false }
        fn is_accel_state(&self, _sid: StateID) -> bool { true }
        fn match_len(&self, _sid: StateID) -> usize { 2 }
        fn match_pattern(&self, _sid: StateID, _index: usize) -> PatternID { PatternID::default() }
        fn next_state(&self, _sid: StateID, _byte: u8) -> StateID { StateID::default() }
        fn accelerator(&self, _sid: StateID) -> &[u8] { &[1, 2] }
    }

    let haystack = b"hello world";
    let input = Input::new(&haystack).set_span(0..10);
    let mut state = OverlappingState {
        mat: None,
        id: Some(StateID::default()),
        at: 9, // greater than input.start()
        next_match_index: Some(2), // equals match_len
        rev_eoi: false,
    };

    let dfa = MockDFA;
    let _result = find_overlapping_rev(&dfa, &input, &mut state);
}

