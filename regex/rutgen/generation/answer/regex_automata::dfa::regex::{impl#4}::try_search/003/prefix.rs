// Answer 0

#[test]
fn test_try_search_success_with_anchored() {
    struct TestAutomaton;

    impl Automaton for TestAutomaton {
        fn next_state(&self, current: StateID, input: u8) -> StateID { unimplemented!() }
        unsafe fn next_state_unchecked(&self, current: StateID, input: u8) -> StateID { unimplemented!() }
        fn next_eoi_state(&self, current: StateID) -> StateID { unimplemented!() }
        fn start_state(&self, config: &start::Config) -> Result<StateID, StartError> { unimplemented!() }
        fn start_state_forward(&self, input: &Input<'_>) -> Result<StateID, MatchError> { unimplemented!() }
        fn start_state_reverse(&self, input: &Input<'_>) -> Result<StateID, MatchError> { unimplemented!() }
        fn universal_start_state(&self, _mode: Anchored) -> Option<StateID> { unimplemented!() }
        fn is_special_state(&self, id: StateID) -> bool { false }
        fn is_dead_state(&self, id: StateID) -> bool { false }
        fn is_quit_state(&self, id: StateID) -> bool { false }
        fn is_match_state(&self, id: StateID) -> bool { true }
        fn is_start_state(&self, id: StateID) -> bool { false }
        fn is_accel_state(&self, id: StateID) -> bool { false }
        fn pattern_len(&self) -> usize { 1 }
        fn match_len(&self, id: StateID) -> usize { 1 }
        fn match_pattern(&self, id: StateID, index: usize) -> PatternID { PatternID(0) }
        fn has_empty(&self) -> bool { false }
        fn is_utf8(&self) -> bool { true }
        fn is_always_start_anchored(&self) -> bool { false }
        fn accelerator(&self, _id: StateID) -> &[u8] { &[] }
        fn get_prefilter(&self) -> Option<&Prefilter> { None }
        fn try_search_fwd(&self, input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> {
            Ok(Some(HalfMatch::new(PatternID(0), input.start() + 1)))
        }
        fn try_search_rev(&self, input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { unimplemented!() }
    }

    let haystack = vec![b'a'];
    let input = Input::new(&haystack).span(0..1).anchored(Anchored::Yes).earliest(true);
    let automaton = TestAutomaton;
    let regex = Regex { automaton };

    let result = regex.try_search(&input);
}

#[test]
fn test_try_search_no_match() {
    struct TestAutomaton;

    impl Automaton for TestAutomaton {
        fn next_state(&self, current: StateID, input: u8) -> StateID { unimplemented!() }
        unsafe fn next_state_unchecked(&self, current: StateID, input: u8) -> StateID { unimplemented!() }
        fn next_eoi_state(&self, current: StateID) -> StateID { unimplemented!() }
        fn start_state(&self, config: &start::Config) -> Result<StateID, StartError> { unimplemented!() }
        fn start_state_forward(&self, input: &Input<'_>) -> Result<StateID, MatchError> { unimplemented!() }
        fn start_state_reverse(&self, input: &Input<'_>) -> Result<StateID, MatchError> { unimplemented!() }
        fn universal_start_state(&self, _mode: Anchored) -> Option<StateID> { unimplemented!() }
        fn is_special_state(&self, id: StateID) -> bool { false }
        fn is_dead_state(&self, id: StateID) -> bool { false }
        fn is_quit_state(&self, id: StateID) -> bool { false }
        fn is_match_state(&self, id: StateID) -> bool { false }
        fn is_start_state(&self, id: StateID) -> bool { false }
        fn is_accel_state(&self, id: StateID) -> bool { false }
        fn pattern_len(&self) -> usize { 1 }
        fn match_len(&self, id: StateID) -> usize { 0 }
        fn match_pattern(&self, id: StateID, index: usize) -> PatternID { PatternID(0) }
        fn has_empty(&self) -> bool { false }
        fn is_utf8(&self) -> bool { true }
        fn is_always_start_anchored(&self) -> bool { false }
        fn accelerator(&self, _id: StateID) -> &[u8] { &[] }
        fn get_prefilter(&self) -> Option<&Prefilter> { None }
        fn try_search_fwd(&self, input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> {
            Ok(None)
        }
        fn try_search_rev(&self, input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { unimplemented!() }
    }

    let haystack = vec![b'a'];
    let input = Input::new(&haystack).span(0..1).anchored(Anchored::Yes).earliest(true);
    let automaton = TestAutomaton;
    let regex = Regex { automaton };

    let result = regex.try_search(&input);
}

