// Answer 0

#[test]
fn test_eoi_rev_valid_match() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 2,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 64,
    };
    
    let mut cache = Cache {
        trans: vec![LazyStateID::new_unchecked(1)],
        starts: vec![LazyStateID::new_unchecked(0)],
        states: vec![],
        states_to_id: StateMap::default(),
        sparses: SparseSets::default(),
        stack: vec![],
        scratch_state_builder: StateBuilderEmpty::default(),
        state_saver: StateSaver::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };
    
    let input = Input::new(&b"example"[..])
        .span(Span { start: 1, end: 7 })  // sp.start > 0
        .anchored(Anchored::None)
        .earliest(true);
    
    let mut sid = LazyStateID::new_unchecked(0);
    let mut mat: Option<HalfMatch> = None;

    let result = eoi_rev(&dfa, &mut cache, &input, &mut sid, &mut mat);
}

#[test]
fn test_eoi_rev_match_found() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 2,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 64,
    };

    let mut cache = Cache {
        trans: vec![LazyStateID::new_unchecked(0), LazyStateID::new_unchecked(1)],
        starts: vec![LazyStateID::new_unchecked(0)],
        states: vec![],
        states_to_id: StateMap::default(),
        sparses: SparseSets::default(),
        stack: vec![],
        scratch_state_builder: StateBuilderEmpty::default(),
        state_saver: StateSaver::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };

    let input = Input::new(&b"example"[..])
        .span(Span { start: 2, end: 7 })  // sp.start > 0
        .anchored(Anchored::None)
        .earliest(true);
    
    let mut sid = LazyStateID::new_unchecked(1); // Assuming this state ID will lead to a match
    let mut mat: Option<HalfMatch> = None;

    let result = eoi_rev(&dfa, &mut cache, &input, &mut sid, &mut mat);
}

