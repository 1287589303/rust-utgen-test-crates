// Answer 0

#[test]
fn test_try_search_overlapping_fwd_case_1() {
    struct TestAutomaton;

    impl Automaton for TestAutomaton {
        fn next_state(&self, _: StateID, _: u8) -> StateID { unimplemented!() }
        unsafe fn next_state_unchecked(&self, _: StateID, _: u8) -> StateID { unimplemented!() }
        fn next_eoi_state(&self, _: StateID) -> StateID { unimplemented!() }
        fn start_state(&self, _: &start::Config) -> Result<StateID, StartError> { unimplemented!() }
        fn start_state_forward(&self, _: &Input<'_>) -> Result<StateID, MatchError> { unimplemented!() }
        fn start_state_reverse(&self, _: &Input<'_>) -> Result<StateID, MatchError> { unimplemented!() }
        fn universal_start_state(&self, _: Anchored) -> Option<StateID> { unimplemented!() }
        fn is_special_state(&self, _: StateID) -> bool { unimplemented!() }
        fn is_dead_state(&self, _: StateID) -> bool { false }
        fn is_quit_state(&self, _: StateID) -> bool { false }
        fn is_match_state(&self, _: StateID) -> bool { unimplemented!() }
        fn is_start_state(&self, _: StateID) -> bool { unimplemented!() }
        fn is_accel_state(&self, _: StateID) -> bool { unimplemented!() }
        fn pattern_len(&self) -> usize { 2 }
        fn match_len(&self, _: StateID) -> usize { 4 }
        fn match_pattern(&self, _: StateID, _: usize) -> PatternID { 0 }
        fn has_empty(&self) -> bool { false }
        fn is_utf8(&self) -> bool { true }
        fn is_always_start_anchored(&self) -> bool { false }
        fn accelerator(&self, _: StateID) -> &[u8] { unimplemented!() }
        fn get_prefilter(&self) -> Option<&Prefilter> { unimplemented!() }
        fn try_search_fwd(&self, _: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { 
            Ok(Some(HalfMatch { pattern: 0, offset: 0 })) 
        }
        fn try_search_rev(&self, _: &Input<'_>, _: &mut OverlappingState) -> Result<(), MatchError> { unimplemented!() }
    }

    let automaton = TestAutomaton;
    let input = Input::new(b"@foo");
    let mut state = OverlappingState::start();

    let result = automaton.try_search_overlapping_fwd(&input, &mut state);
    let _ = state.get_match();
}

#[test]
fn test_try_search_overlapping_fwd_case_2() {
    struct TestAutomaton;

    impl Automaton for TestAutomaton {
        fn next_state(&self, _: StateID, _: u8) -> StateID { unimplemented!() }
        unsafe fn next_state_unchecked(&self, _: StateID, _: u8) -> StateID { unimplemented!() }
        fn next_eoi_state(&self, _: StateID) -> StateID { unimplemented!() }
        fn start_state(&self, _: &start::Config) -> Result<StateID, StartError> { unimplemented!() }
        fn start_state_forward(&self, _: &Input<'_>) -> Result<StateID, MatchError> { unimplemented!() }
        fn start_state_reverse(&self, _: &Input<'_>) -> Result<StateID, MatchError> { unimplemented!() }
        fn universal_start_state(&self, _: Anchored) -> Option<StateID> { unimplemented!() }
        fn is_special_state(&self, _: StateID) -> bool { unimplemented!() }
        fn is_dead_state(&self, _: StateID) -> bool { false }
        fn is_quit_state(&self, _: StateID) -> bool { false }
        fn is_match_state(&self, _: StateID) -> bool { unimplemented!() }
        fn is_start_state(&self, _: StateID) -> bool { unimplemented!() }
        fn is_accel_state(&self, _: StateID) -> bool { unimplemented!() }
        fn pattern_len(&self) -> usize { 2 }
        fn match_len(&self, _: StateID) -> usize { 5 }
        fn match_pattern(&self, _: StateID, _: usize) -> PatternID { 1 }
        fn has_empty(&self) -> bool { false }
        fn is_utf8(&self) -> bool { true }
        fn is_always_start_anchored(&self) -> bool { false }
        fn accelerator(&self, _: StateID) -> &[u8] { unimplemented!() }
        fn get_prefilter(&self) -> Option<&Prefilter> { unimplemented!() }
        fn try_search_fwd(&self, _: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { 
            Ok(Some(HalfMatch { pattern: 1, offset: 4 })) 
        }
        fn try_search_rev(&self, _: &Input<'_>, _: &mut OverlappingState) -> Result<(), MatchError> { unimplemented!() }
    }

    let automaton = TestAutomaton;
    let input = Input::new(b"@foo bar");
    let mut state = OverlappingState::start();

    let result = automaton.try_search_overlapping_fwd(&input, &mut state);
    let _ = state.get_match();
}

