// Answer 0

#[test]
fn test_pattern_len_with_multiple_patterns() {
    struct DummyAutomaton;

    impl Automaton for DummyAutomaton {
        fn next_state(&self, _: StateID, _: u8) -> StateID { 0 }
        unsafe fn next_state_unchecked(&self, _: StateID, _: u8) -> StateID { 0 }
        fn next_eoi_state(&self, _: StateID) -> StateID { 0 }
        fn start_state(&self, _: &start::Config) -> Result<StateID, StartError> { Ok(0) }
        fn start_state_forward(&self, _: &Input<'_>) -> Result<StateID, MatchError> { Ok(0) }
        fn start_state_reverse(&self, _: &Input<'_>) -> Result<StateID, MatchError> { Ok(0) }
        fn universal_start_state(&self, _: Anchored) -> Option<StateID> { Some(0) }
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
        fn is_utf8(&self) -> bool { true }
        fn is_always_start_anchored(&self) -> bool { false }
        fn accelerator(&self, _: StateID) -> &[u8] { &[] }
        fn get_prefilter(&self) -> Option<&Prefilter> { None }
        fn try_search_fwd(&self, _: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { Ok(None) }
        fn try_search_rev(&self, _: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { Ok(None) }
        fn try_search_overlapping_fwd(
            &self,
            _: &Input<'_>,
            _: &mut OverlappingState,
        ) -> Result<(), MatchError> { Ok(()) }
        fn try_search_overlapping_rev(
            &self,
            _: &Input<'_>,
            _: &mut OverlappingState,
        ) -> Result<(), MatchError> { Ok(()) }
    }

    let re = Regex::new_many(&[r"[a-z]+", r"[0-9]+", r"\w+"]).unwrap();
    let length = re.pattern_len();
}

#[test]
fn test_pattern_len_with_empty_patterns() {
    struct DummyAutomaton;

    impl Automaton for DummyAutomaton {
        fn next_state(&self, _: StateID, _: u8) -> StateID { 0 }
        unsafe fn next_state_unchecked(&self, _: StateID, _: u8) -> StateID { 0 }
        fn next_eoi_state(&self, _: StateID) -> StateID { 0 }
        fn start_state(&self, _: &start::Config) -> Result<StateID, StartError> { Ok(0) }
        fn start_state_forward(&self, _: &Input<'_>) -> Result<StateID, MatchError> { Ok(0) }
        fn start_state_reverse(&self, _: &Input<'_>) -> Result<StateID, MatchError> { Ok(0) }
        fn universal_start_state(&self, _: Anchored) -> Option<StateID> { Some(0) }
        fn is_special_state(&self, _: StateID) -> bool { false }
        fn is_dead_state(&self, _: StateID) -> bool { false }
        fn is_quit_state(&self, _: StateID) -> bool { false }
        fn is_match_state(&self, _: StateID) -> bool { false }
        fn is_start_state(&self, _: StateID) -> bool { true }
        fn is_accel_state(&self, _: StateID) -> bool { false }
        fn pattern_len(&self) -> usize { 0 }
        fn match_len(&self, _: StateID) -> usize { 0 }
        fn match_pattern(&self, _: StateID, _: usize) -> PatternID { 0 }
        fn has_empty(&self) -> bool { true }
        fn is_utf8(&self) -> bool { true }
        fn is_always_start_anchored(&self) -> bool { false }
        fn accelerator(&self, _: StateID) -> &[u8] { &[] }
        fn get_prefilter(&self) -> Option<&Prefilter> { None }
        fn try_search_fwd(&self, _: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { Ok(None) }
        fn try_search_rev(&self, _: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { Ok(None) }
        fn try_search_overlapping_fwd(
            &self,
            _: &Input<'_>,
            _: &mut OverlappingState,
        ) -> Result<(), MatchError> { Ok(()) }
        fn try_search_overlapping_rev(
            &self,
            _: &Input<'_>,
            _: &mut OverlappingState,
        ) -> Result<(), MatchError> { Ok(()) }
    }

    let re = Regex::new_many(&[]).unwrap();
    let length = re.pattern_len();
}

#[test]
fn test_pattern_len_with_max_length_patterns() {
    struct DummyAutomaton;

    impl Automaton for DummyAutomaton {
        fn next_state(&self, _: StateID, _: u8) -> StateID { 0 }
        unsafe fn next_state_unchecked(&self, _: StateID, _: u8) -> StateID { 0 }
        fn next_eoi_state(&self, _: StateID) -> StateID { 0 }
        fn start_state(&self, _: &start::Config) -> Result<StateID, StartError> { Ok(0) }
        fn start_state_forward(&self, _: &Input<'_>) -> Result<StateID, MatchError> { Ok(0) }
        fn start_state_reverse(&self, _: &Input<'_>) -> Result<StateID, MatchError> { Ok(0) }
        fn universal_start_state(&self, _: Anchored) -> Option<StateID> { Some(0) }
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
        fn is_utf8(&self) -> bool { true }
        fn is_always_start_anchored(&self) -> bool { false }
        fn accelerator(&self, _: StateID) -> &[u8] { &[] }
        fn get_prefilter(&self) -> Option<&Prefilter> { None }
        fn try_search_fwd(&self, _: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { Ok(None) }
        fn try_search_rev(&self, _: &Input<'_>) -> Result<Option<HalfMatch>, MatchError> { Ok(None) }
        fn try_search_overlapping_fwd(
            &self,
            _: &Input<'_>,
            _: &mut OverlappingState,
        ) -> Result<(), MatchError> { Ok(()) }
        fn try_search_overlapping_rev(
            &self,
            _: &Input<'_>,
            _: &mut OverlappingState,
        ) -> Result<(), MatchError> { Ok(()) }
    }

    let re = Regex::new_many(&[r"[a-z]{100}", r"[0-9]{100}", r"\w{100}"]).unwrap();
    let length = re.pattern_len();
}

