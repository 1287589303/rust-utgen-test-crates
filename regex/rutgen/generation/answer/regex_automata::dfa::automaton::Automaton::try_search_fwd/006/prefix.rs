// Answer 0

#[test]
fn test_try_search_fwd_some_match() {
    struct TestAutomaton;

    unsafe impl Automaton for TestAutomaton {
        fn next_state(&self, _current: StateID, _input: u8) -> StateID { 0 }
        unsafe fn next_state_unchecked(&self, _current: StateID, _input: u8) -> StateID { 0 }
        fn next_eoi_state(&self, _current: StateID) -> StateID { 0 }
        fn start_state(&self, _config: &start::Config) -> Result<StateID, StartError> { Ok(0) }
        fn start_state_forward(&self, _input: &Input<'_>) -> Result<StateID, MatchError> { Ok(0) }
        fn start_state_reverse(&self, _input: &Input<'_>) -> Result<StateID, MatchError> { Ok(0) }
        fn universal_start_state(&self, _mode: Anchored) -> Option<StateID> { Some(0) }
        fn is_special_state(&self, _id: StateID) -> bool { false }
        fn is_dead_state(&self, _id: StateID) -> bool { false }
        fn is_quit_state(&self, _id: StateID) -> bool { false }
        fn is_match_state(&self, _id: StateID) -> bool { true }
        fn is_start_state(&self, _id: StateID) -> bool { true }
        fn is_accel_state(&self, _id: StateID) -> bool { false }
        fn pattern_len(&self) -> usize { 3 }
        fn match_len(&self, _id: StateID) -> usize { 3 }
        fn match_pattern(&self, _id: StateID, _index: usize) -> PatternID { PatternID::must(0) }
        fn has_empty(&self) -> bool { false }
        fn is_utf8(&self) -> bool { true }
        fn is_always_start_anchored(&self) -> bool { true }
        fn accelerator(&self, _id: StateID) -> &[u8] { &[] }
        fn get_prefilter(&self) -> Option<&Prefilter> { None }
    }

    let automaton = TestAutomaton;
    let input = Input::new(b"foo12345").anchored(Anchored::Unanchored);
    let _ = automaton.try_search_fwd(&input);
}

#[test]
fn test_try_search_fwd_none_match() {
    struct TestAutomaton;

    unsafe impl Automaton for TestAutomaton {
        fn next_state(&self, _current: StateID, _input: u8) -> StateID { 0 }
        unsafe fn next_state_unchecked(&self, _current: StateID, _input: u8) -> StateID { 0 }
        fn next_eoi_state(&self, _current: StateID) -> StateID { 0 }
        fn start_state(&self, _config: &start::Config) -> Result<StateID, StartError> { Ok(0) }
        fn start_state_forward(&self, _input: &Input<'_>) -> Result<StateID, MatchError> { Ok(0) }
        fn start_state_reverse(&self, _input: &Input<'_>) -> Result<StateID, MatchError> { Ok(0) }
        fn universal_start_state(&self, _mode: Anchored) -> Option<StateID> { Some(0) }
        fn is_special_state(&self, _id: StateID) -> bool { false }
        fn is_dead_state(&self, _id: StateID) -> bool { false }
        fn is_quit_state(&self, _id: StateID) -> bool { false }
        fn is_match_state(&self, _id: StateID) -> bool { false }
        fn is_start_state(&self, _id: StateID) -> bool { true }
        fn is_accel_state(&self, _id: StateID) -> bool { false }
        fn pattern_len(&self) -> usize { 3 }
        fn match_len(&self, _id: StateID) -> usize { 3 }
        fn match_pattern(&self, _id: StateID, _index: usize) -> PatternID { PatternID::must(0) }
        fn has_empty(&self) -> bool { false }
        fn is_utf8(&self) -> bool { true }
        fn is_always_start_anchored(&self) -> bool { true }
        fn accelerator(&self, _id: StateID) -> &[u8] { &[] }
        fn get_prefilter(&self) -> Option<&Prefilter> { None }
    }

    let automaton = TestAutomaton;
    let input = Input::new(b"bar").anchored(Anchored::Unanchored);
    let _ = automaton.try_search_fwd(&input);
}

