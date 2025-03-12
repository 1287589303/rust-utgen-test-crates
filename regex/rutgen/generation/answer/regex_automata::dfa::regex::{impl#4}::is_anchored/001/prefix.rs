// Answer 0

#[test]
fn test_is_anchored_with_pattern() {
    struct TestAutomaton;

    impl Automaton for TestAutomaton {
        fn next_state(&self, _current: StateID, _input: u8) -> StateID { 0 }
        unsafe fn next_state_unchecked(&self, _current: StateID, _input: u8) -> StateID { 0 }
        fn next_eoi_state(&self, _current: StateID) -> StateID { 0 }
        fn start_state(&self, _config: &start::Config) -> Result<StateID, StartError> { Ok(0) }
        fn start_state_forward(&self, _input: &Input<'_>) -> Result<StateID, MatchError> { Ok(0) }
        fn start_state_reverse(&self, _input: &Input<'_>) -> Result<StateID, MatchError> { Ok(0) }
        fn universal_start_state(&self, _mode: Anchored) -> Option<StateID> { None }
        fn is_special_state(&self, _id: StateID) -> bool { false }
        fn is_dead_state(&self, _id: StateID) -> bool { false }
        fn is_quit_state(&self, _id: StateID) -> bool { false }
        fn is_match_state(&self, _id: StateID) -> bool { false }
        fn is_start_state(&self, _id: StateID) -> bool { false }
        fn is_accel_state(&self, _id: StateID) -> bool { false }
        fn pattern_len(&self) -> usize { 0 }
        fn match_len(&self, _id: StateID) -> usize { 0 }
        fn match_pattern(&self, _id: StateID, _index: usize) -> PatternID { 0 }
        fn has_empty(&self) -> bool { false }
        fn is_utf8(&self) -> bool { false }
        fn is_always_start_anchored(&self) -> bool { false }
        fn accelerator(&self, _id: StateID) -> &[u8] { &[] }
        fn get_prefilter(&self) -> Option<&Prefilter> { None }
        fn try_search_fwd(&self, _input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { Ok(None) }
        fn try_search_rev(&self, _input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { Ok(None) }
        fn try_search_overlapping_fwd(&self, _input: &Input<'_>, _state: &mut OverlappingState) -> Result<(), MatchError> { Ok(()) }
        fn try_search_overlapping_rev(&self, _input: &Input<'_>, _state: &mut OverlappingState) -> Result<(), MatchError> { Ok(()) }
        #[cfg(feature = "alloc")]
        fn try_which_overlapping_matches(&self, _input: &Input<'_>, _patset: &mut PatternSet) -> Result<(), MatchError> { Ok(()) }
    }

    let automaton = TestAutomaton;
    let pattern_id = 1; // example PatternID

    let input = Input::new(b"example")
        .anchored(Anchored::Pattern(pattern_id));

    let result = automaton.is_anchored(&input);
}

#[test]
fn test_is_anchored_with_no_pattern() {
    struct TestAutomaton;

    impl Automaton for TestAutomaton {
        fn next_state(&self, _current: StateID, _input: u8) -> StateID { 0 }
        unsafe fn next_state_unchecked(&self, _current: StateID, _input: u8) -> StateID { 0 }
        fn next_eoi_state(&self, _current: StateID) -> StateID { 0 }
        fn start_state(&self, _config: &start::Config) -> Result<StateID, StartError> { Ok(0) }
        fn start_state_forward(&self, _input: &Input<'_>) -> Result<StateID, MatchError> { Ok(0) }
        fn start_state_reverse(&self, _input: &Input<'_>) -> Result<StateID, MatchError> { Ok(0) }
        fn universal_start_state(&self, _mode: Anchored) -> Option<StateID> { None }
        fn is_special_state(&self, _id: StateID) -> bool { false }
        fn is_dead_state(&self, _id: StateID) -> bool { false }
        fn is_quit_state(&self, _id: StateID) -> bool { false }
        fn is_match_state(&self, _id: StateID) -> bool { false }
        fn is_start_state(&self, _id: StateID) -> bool { false }
        fn is_accel_state(&self, _id: StateID) -> bool { false }
        fn pattern_len(&self) -> usize { 0 }
        fn match_len(&self, _id: StateID) -> usize { 0 }
        fn match_pattern(&self, _id: StateID, _index: usize) -> PatternID { 0 }
        fn has_empty(&self) -> bool { false }
        fn is_utf8(&self) -> bool { false }
        fn is_always_start_anchored(&self) -> bool { true }
        fn accelerator(&self, _id: StateID) -> &[u8] { &[] }
        fn get_prefilter(&self) -> Option<&Prefilter> { None }
        fn try_search_fwd(&self, _input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { Ok(None) }
        fn try_search_rev(&self, _input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { Ok(None) }
        fn try_search_overlapping_fwd(&self, _input: &Input<'_>, _state: &mut OverlappingState) -> Result<(), MatchError> { Ok(()) }
        fn try_search_overlapping_rev(&self, _input: &Input<'_>, _state: &mut OverlappingState) -> Result<(), MatchError> { Ok(()) }
        #[cfg(feature = "alloc")]
        fn try_which_overlapping_matches(&self, _input: &Input<'_>, _patset: &mut PatternSet) -> Result<(), MatchError> { Ok(()) }
    }

    let automaton = TestAutomaton;

    let input = Input::new(b"example")
        .anchored(Anchored::Pattern(0));

    let result = automaton.is_anchored(&input);
}

