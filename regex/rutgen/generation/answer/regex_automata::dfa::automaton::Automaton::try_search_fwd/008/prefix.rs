// Answer 0

#[test]
fn test_try_search_fwd_no_match() {
    struct TestAutomaton;

    unsafe impl Automaton for TestAutomaton {
        fn next_state(&self, _current: StateID, _input: u8) -> StateID { unimplemented!() }
        unsafe fn next_state_unchecked(&self, _current: StateID, _input: u8) -> StateID { unimplemented!() }
        fn next_eoi_state(&self, _current: StateID) -> StateID { unimplemented!() }
        fn start_state(&self, _config: &start::Config) -> Result<StateID, StartError> { unimplemented!() }
        fn start_state_forward(&self, _input: &Input<'_>) -> Result<StateID, MatchError> { unimplemented!() }
        fn start_state_reverse(&self, _input: &Input<'_>) -> Result<StateID, MatchError> { unimplemented!() }
        #[inline]
        fn universal_start_state(&self, _mode: Anchored) -> Option<StateID> { None }
        fn is_special_state(&self, _id: StateID) -> bool { false }
        fn is_dead_state(&self, _id: StateID) -> bool { false }
        fn is_quit_state(&self, _id: StateID) -> bool { false }
        fn is_match_state(&self, _id: StateID) -> bool { false }
        fn is_start_state(&self, _id: StateID) -> bool { false }
        fn is_accel_state(&self, _id: StateID) -> bool { false }
        fn pattern_len(&self) -> usize { 5 }
        fn match_len(&self, _id: StateID) -> usize { 0 }
        fn match_pattern(&self, _id: StateID, _index: usize) -> PatternID { PatternID::must(0) }
        fn has_empty(&self) -> bool { false }
        fn is_utf8(&self) -> bool { true }
        fn is_always_start_anchored(&self) -> bool { false }
        #[inline]
        fn accelerator(&self, _id: StateID) -> &[u8] { &[] }
        #[inline]
        fn get_prefilter(&self) -> Option<&Prefilter> { None }
    }

    let automaton = TestAutomaton;
    let input = Input {
        haystack: b"sample input",
        span: Span::new(..),
        anchored: Anchored::None,
        earliest: false,
    };

    let _ = automaton.try_search_fwd(&input);
}

#[test]
fn test_try_search_fwd_some_match() {
    struct TestAutomaton;

    unsafe impl Automaton for TestAutomaton {
        fn next_state(&self, _current: StateID, _input: u8) -> StateID { unimplemented!() }
        unsafe fn next_state_unchecked(&self, _current: StateID, _input: u8) -> StateID { unimplemented!() }
        fn next_eoi_state(&self, _current: StateID) -> StateID { unimplemented!() }
        fn start_state(&self, _config: &start::Config) -> Result<StateID, StartError> { unimplemented!() }
        fn start_state_forward(&self, _input: &Input<'_>) -> Result<StateID, MatchError> { unimplemented!() }
        fn start_state_reverse(&self, _input: &Input<'_>) -> Result<StateID, MatchError> { unimplemented!() }
        #[inline]
        fn universal_start_state(&self, _mode: Anchored) -> Option<StateID> { None }
        fn is_special_state(&self, _id: StateID) -> bool { false }
        fn is_dead_state(&self, _id: StateID) -> bool { false }
        fn is_quit_state(&self, _id: StateID) -> bool { false }
        fn is_match_state(&self, _id: StateID) -> bool { true }
        fn is_start_state(&self, _id: StateID) -> bool { false }
        fn is_accel_state(&self, _id: StateID) -> bool { false }
        fn pattern_len(&self) -> usize { 5 }
        fn match_len(&self, _id: StateID) -> usize { 5 }
        fn match_pattern(&self, _id: StateID, _index: usize) -> PatternID { PatternID::must(0) }
        fn has_empty(&self) -> bool { false }
        fn is_utf8(&self) -> bool { true }
        fn is_always_start_anchored(&self) -> bool { false }
        #[inline]
        fn accelerator(&self, _id: StateID) -> &[u8] { &[] }
        #[inline]
        fn get_prefilter(&self) -> Option<&Prefilter> { None }
    }

    let automaton = TestAutomaton;
    let input = Input {
        haystack: b"matching pattern",
        span: Span::new(..),
        anchored: Anchored::None,
        earliest: false,
    };

    let _ = automaton.try_search_fwd(&input);
}

