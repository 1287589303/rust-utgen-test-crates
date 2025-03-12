// Answer 0

#[test]
fn test_try_which_overlapping_matches_full_patterns() {
    struct TestAutomaton;

    unsafe impl Automaton for TestAutomaton {
        // implementations of required methods that ensure the test passes
        fn next_state(&self, _: StateID, _: u8) -> StateID { unimplemented!() }
        unsafe fn next_state_unchecked(&self, _: StateID, _: u8) -> StateID { unimplemented!() }
        fn next_eoi_state(&self, _: StateID) -> StateID { unimplemented!() }
        fn start_state(&self, _: &start::Config) -> Result<StateID, StartError> { unimplemented!() }
        fn start_state_forward(&self, _: &Input<'_>) -> Result<StateID, MatchError> { unimplemented!() }
        fn start_state_reverse(&self, _: &Input<'_>) -> Result<StateID, MatchError> { unimplemented!() }
        fn universal_start_state(&self, _: Anchored) -> Option<StateID> { unimplemented!() }
        fn is_special_state(&self, _: StateID) -> bool { unimplemented!() }
        fn is_dead_state(&self, _: StateID) -> bool { unimplemented!() }
        fn is_quit_state(&self, _: StateID) -> bool { unimplemented!() }
        fn is_match_state(&self, _: StateID) -> bool { unimplemented!() }
        fn is_start_state(&self, _: StateID) -> bool { unimplemented!() }
        fn is_accel_state(&self, _: StateID) -> bool { unimplemented!() }
        fn pattern_len(&self) -> usize { 5 }
        fn match_len(&self, _: StateID) -> usize { unimplemented!() }
        fn match_pattern(&self, _: StateID, _: usize) -> PatternID { unimplemented!() }
        fn has_empty(&self) -> bool { false }
        fn is_utf8(&self) -> bool { true }
        fn is_always_start_anchored(&self) -> bool { false }
        fn accelerator(&self, _: StateID) -> &[u8] { unimplemented!() }
        fn get_prefilter(&self) -> Option<&Prefilter> { unimplemented!() }
        fn try_search_fwd(&self, _: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { 
            Ok(Some(HalfMatch::new(PatternID(0), 0))) 
        }
        fn try_search_rev(&self, _: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { unimplemented!() }
    }

    let patterns = &[b"foo", b"bar", b"baz", b"qux"];
    let dfa = TestAutomaton;
    let input = Input::new(b"foobar");
    let mut patset = PatternSet::new(dfa.pattern_len());
    dfa.try_which_overlapping_matches(&input, &mut patset).unwrap();
}

#[test]
fn test_try_which_overlapping_matches_empty_haystack() {
    struct TestAutomaton;

    unsafe impl Automaton for TestAutomaton {
        fn next_state(&self, _: StateID, _: u8) -> StateID { unimplemented!() }
        unsafe fn next_state_unchecked(&self, _: StateID, _: u8) -> StateID { unimplemented!() }
        fn next_eoi_state(&self, _: StateID) -> StateID { unimplemented!() }
        fn start_state(&self, _: &start::Config) -> Result<StateID, StartError> { unimplemented!() }
        fn start_state_forward(&self, _: &Input<'_>) -> Result<StateID, MatchError> { unimplemented!() }
        fn start_state_reverse(&self, _: &Input<'_>) -> Result<StateID, MatchError> { unimplemented!() }
        fn universal_start_state(&self, _: Anchored) -> Option<StateID> { unimplemented!() }
        fn is_special_state(&self, _: StateID) -> bool { unimplemented!() }
        fn is_dead_state(&self, _: StateID) -> bool { unimplemented!() }
        fn is_quit_state(&self, _: StateID) -> bool { unimplemented!() }
        fn is_match_state(&self, _: StateID) -> bool { unimplemented!() }
        fn is_start_state(&self, _: StateID) -> bool { unimplemented!() }
        fn is_accel_state(&self, _: StateID) -> bool { unimplemented!() }
        fn pattern_len(&self) -> usize { 3 }
        fn match_len(&self, _: StateID) -> usize { unimplemented!() }
        fn match_pattern(&self, _: StateID, _: usize) -> PatternID { unimplemented!() }
        fn has_empty(&self) -> bool { false }
        fn is_utf8(&self) -> bool { true }
        fn is_always_start_anchored(&self) -> bool { false }
        fn accelerator(&self, _: StateID) -> &[u8] { unimplemented!() }
        fn get_prefilter(&self) -> Option<&Prefilter> { unimplemented!() }
        fn try_search_fwd(&self, _: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { 
            Ok(None) 
        }
        fn try_search_rev(&self, _: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { unimplemented!() }
    }

    let dfa = TestAutomaton;
    let input = Input::new(b"");
    let mut patset = PatternSet::new(dfa.pattern_len());
    dfa.try_which_overlapping_matches(&input, &mut patset).unwrap();
}

#[test]
fn test_try_which_overlapping_matches_not_full() {
    struct TestAutomaton;

    unsafe impl Automaton for TestAutomaton {
        fn next_state(&self, _: StateID, _: u8) -> StateID { unimplemented!() }
        unsafe fn next_state_unchecked(&self, _: StateID, _: u8) -> StateID { unimplemented!() }
        fn next_eoi_state(&self, _: StateID) -> StateID { unimplemented!() }
        fn start_state(&self, _: &start::Config) -> Result<StateID, StartError> { unimplemented!() }
        fn start_state_forward(&self, _: &Input<'_>) -> Result<StateID, MatchError> { unimplemented!() }
        fn start_state_reverse(&self, _: &Input<'_>) -> Result<StateID, MatchError> { unimplemented!() }
        fn universal_start_state(&self, _: Anchored) -> Option<StateID> { unimplemented!() }
        fn is_special_state(&self, _: StateID) -> bool { unimplemented!() }
        fn is_dead_state(&self, _: StateID) -> bool { unimplemented!() }
        fn is_quit_state(&self, _: StateID) -> bool { unimplemented!() }
        fn is_match_state(&self, _: StateID) -> bool { unimplemented!() }
        fn is_start_state(&self, _: StateID) -> bool { unimplemented!() }
        fn is_accel_state(&self, _: StateID) -> bool { unimplemented!() }
        fn pattern_len(&self) -> usize { 4 }
        fn match_len(&self, _: StateID) -> usize { unimplemented!() }
        fn match_pattern(&self, _: StateID, _: usize) -> PatternID { unimplemented!() }
        fn has_empty(&self) -> bool { false }
        fn is_utf8(&self) -> bool { true }
        fn is_always_start_anchored(&self) -> bool { false }
        fn accelerator(&self, _: StateID) -> &[u8] { unimplemented!() }
        fn get_prefilter(&self) -> Option<&Prefilter> { unimplemented!() }
        fn try_search_fwd(&self, _: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { 
            Ok(Some(HalfMatch::new(PatternID(0), 0))) 
        }
        fn try_search_rev(&self, _: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { unimplemented!() }
    }

    let dfa = TestAutomaton;
    let input = Input::new(b"foobar");
    let mut patset = PatternSet::new(dfa.pattern_len());
    dfa.try_which_overlapping_matches(&input, &mut patset).unwrap();
}

