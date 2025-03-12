// Answer 0

#[test]
fn test_dfa_always_match_no_patterns() {
    struct EmptyDFA;

    impl Automaton for EmptyDFA {
        fn next_state(&self, _: StateID, _: u8) -> StateID { 0 }
        unsafe fn next_state_unchecked(&self, _: StateID, _: u8) -> StateID { 0 }
        fn next_eoi_state(&self, _: StateID) -> StateID { 0 }
        fn start_state(&self, _: &start::Config) -> Result<StateID, StartError> { Ok(0) }
        fn start_state_forward(&self, _: &Input<'_>) -> Result<StateID, MatchError> { Err(MatchError::new()) }
        fn start_state_reverse(&self, _: &Input<'_>) -> Result<StateID, MatchError> { Err(MatchError::new()) }
        fn universal_start_state(&self, _: Anchored) -> Option<StateID> { None }
        fn is_special_state(&self, _: StateID) -> bool { false }
        fn is_dead_state(&self, _: StateID) -> bool { true }
        fn is_quit_state(&self, _: StateID) -> bool { false }
        fn is_match_state(&self, _: StateID) -> bool { false }
        fn is_start_state(&self, _: StateID) -> bool { false }
        fn is_accel_state(&self, _: StateID) -> bool { false }
        fn pattern_len(&self) -> usize { 0 }
        fn match_len(&self, _: StateID) -> usize { 0 }
        fn match_pattern(&self, _: StateID, _: usize) -> PatternID { 0 }
        fn has_empty(&self) -> bool { false }
        fn is_utf8(&self) -> bool { false }
        fn is_always_start_anchored(&self) -> bool { false }
        fn accelerator(&self, _: StateID) -> &[u8] { &[] }
        fn get_prefilter(&self) -> Option<&Prefilter> { None }
        fn try_search_fwd(&self, _: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { Ok(None) }
        fn try_search_rev(&self, _: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { Ok(None) }
        fn try_search_overlapping_fwd(&self, _: &Input<'_>, _: &mut OverlappingState) -> Result<(), MatchError> { Err(MatchError::new()) }
        fn try_search_overlapping_rev(&self, _: &Input<'_>, _: &mut OverlappingState) -> Result<(), MatchError> { Err(MatchError::new()) }
    }

    let result = EmptyDFA::always_match();
}

#[test]
fn test_dfa_always_match_single_pattern() {
    struct SinglePatternDFA;

    impl Automaton for SinglePatternDFA {
        fn next_state(&self, _: StateID, _: u8) -> StateID { 0 }
        unsafe fn next_state_unchecked(&self, _: StateID, _: u8) -> StateID { 0 }
        fn next_eoi_state(&self, _: StateID) -> StateID { 0 }
        fn start_state(&self, _: &start::Config) -> Result<StateID, StartError> { Ok(0) }
        fn start_state_forward(&self, _: &Input<'_>) -> Result<StateID, MatchError> { Err(MatchError::new()) }
        fn start_state_reverse(&self, _: &Input<'_>) -> Result<StateID, MatchError> { Err(MatchError::new()) }
        fn universal_start_state(&self, _: Anchored) -> Option<StateID> { None }
        fn is_special_state(&self, _: StateID) -> bool { false }
        fn is_dead_state(&self, _: StateID) -> bool { false }
        fn is_quit_state(&self, _: StateID) -> bool { false }
        fn is_match_state(&self, _: StateID) -> bool { true }
        fn is_start_state(&self, _: StateID) -> bool { true }
        fn is_accel_state(&self, _: StateID) -> bool { false }
        fn pattern_len(&self) -> usize { 1 }
        fn match_len(&self, _: StateID) -> usize { 0 }
        fn match_pattern(&self, _: StateID, _: usize) -> PatternID { 0 }
        fn has_empty(&self) -> bool { false }
        fn is_utf8(&self) -> bool { false }
        fn is_always_start_anchored(&self) -> bool { false }
        fn accelerator(&self, _: StateID) -> &[u8] { &[] }
        fn get_prefilter(&self) -> Option<&Prefilter> { None }
        fn try_search_fwd(&self, _: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { Ok(None) }
        fn try_search_rev(&self, _: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { Ok(None) }
        fn try_search_overlapping_fwd(&self, _: &Input<'_>, _: &mut OverlappingState) -> Result<(), MatchError> { Err(MatchError::new()) }
        fn try_search_overlapping_rev(&self, _: &Input<'_>, _: &mut OverlappingState) -> Result<(), MatchError> { Err(MatchError::new()) }
    }

    let result = SinglePatternDFA::always_match();
}

#[test]
fn test_dfa_always_match_multiple_patterns() {
    struct MultiplePatternsDFA;

    impl Automaton for MultiplePatternsDFA {
        fn next_state(&self, _: StateID, _: u8) -> StateID { 0 }
        unsafe fn next_state_unchecked(&self, _: StateID, _: u8) -> StateID { 0 }
        fn next_eoi_state(&self, _: StateID) -> StateID { 0 }
        fn start_state(&self, _: &start::Config) -> Result<StateID, StartError> { Ok(0) }
        fn start_state_forward(&self, _: &Input<'_>) -> Result<StateID, MatchError> { Err(MatchError::new()) }
        fn start_state_reverse(&self, _: &Input<'_>) -> Result<StateID, MatchError> { Err(MatchError::new()) }
        fn universal_start_state(&self, _: Anchored) -> Option<StateID> { None }
        fn is_special_state(&self, _: StateID) -> bool { false }
        fn is_dead_state(&self, _: StateID) -> bool { false }
        fn is_quit_state(&self, _: StateID) -> bool { false }
        fn is_match_state(&self, _: StateID) -> bool { false }
        fn is_start_state(&self, _: StateID) -> bool { true }
        fn is_accel_state(&self, _: StateID) -> bool { false }
        fn pattern_len(&self) -> usize { 3 }
        fn match_len(&self, _: StateID) -> usize { 0 }
        fn match_pattern(&self, _: StateID, _: usize) -> PatternID { 0 }
        fn has_empty(&self) -> bool { false }
        fn is_utf8(&self) -> bool { false }
        fn is_always_start_anchored(&self) -> bool { false }
        fn accelerator(&self, _: StateID) -> &[u8] { &[] }
        fn get_prefilter(&self) -> Option<&Prefilter> { None }
        fn try_search_fwd(&self, _: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { Ok(None) }
        fn try_search_rev(&self, _: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { Ok(None) }
        fn try_search_overlapping_fwd(&self, _: &Input<'_>, _: &mut OverlappingState) -> Result<(), MatchError> { Err(MatchError::new()) }
        fn try_search_overlapping_rev(&self, _: &Input<'_>, _: &mut OverlappingState) -> Result<(), MatchError> { Err(MatchError::new()) }
    }

    let result = MultiplePatternsDFA::always_match();
}

