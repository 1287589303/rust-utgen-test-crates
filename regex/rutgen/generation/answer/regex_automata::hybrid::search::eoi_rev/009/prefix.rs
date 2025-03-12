// Answer 0

#[test]
fn test_eoi_rev_case_1() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 1024,
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

    let input = Input::new(&b"abc"[..])
        .span(Span { start: 0, end: 3 });

    let mut sid = LazyStateID::new_unchecked(2);
    let mut mat = None;

    let result = unsafe { eoi_rev(&dfa, &mut cache, &input, &mut sid, &mut mat) };
}

#[test]
fn test_eoi_rev_case_2() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 1024,
    };

    let mut cache = Cache {
        trans: vec![LazyStateID::new_unchecked(3)],
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

    let input = Input::new(&b"xyz"[..])
        .span(Span { start: 0, end: 3 });

    let mut sid = LazyStateID::new_unchecked(4);
    let mut mat = None;

    let result = unsafe { eoi_rev(&dfa, &mut cache, &input, &mut sid, &mut mat) };
}

