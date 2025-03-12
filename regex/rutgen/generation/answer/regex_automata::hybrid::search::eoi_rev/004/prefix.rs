// Answer 0

#[test]
fn test_eoi_rev_case_1() {
    let haystack = b"example";
    let span = Span { start: 1, end: 7 };
    let input = Input::new(&haystack).span(span);
    
    let dfa = DFA {
        config: Default::default(),
        nfa: Default::default(),
        stride2: Default::default(),
        start_map: Default::default(),
        classes: Default::default(),
        quitset: Default::default(),
        cache_capacity: Default::default(),
    };
    
    let mut cache = Cache {
        trans: vec![LazyStateID::new_unchecked(0)],
        starts: vec![LazyStateID::new_unchecked(0)],
        states: vec![],
        states_to_id: Default::default(),
        sparses: Default::default(),
        stack: vec![],
        scratch_state_builder: Default::default(),
        state_saver: Default::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };
    
    let mut sid = LazyStateID::new_unchecked(1);
    let mut mat = None;

    let _ = eoi_rev(&dfa, &mut cache, &input, &mut sid, &mut mat);
}

#[test]
fn test_eoi_rev_case_2() {
    let haystack = b"teststring";
    let span = Span { start: 1, end: 10 };
    let input = Input::new(&haystack).span(span);
    
    let dfa = DFA {
        config: Default::default(),
        nfa: Default::default(),
        stride2: Default::default(),
        start_map: Default::default(),
        classes: Default::default(),
        quitset: Default::default(),
        cache_capacity: Default::default(),
    };
    
    let mut cache = Cache {
        trans: vec![LazyStateID::new_unchecked(2)],
        starts: vec![LazyStateID::new_unchecked(2)],
        states: vec![],
        states_to_id: Default::default(),
        sparses: Default::default(),
        stack: vec![],
        scratch_state_builder: Default::default(),
        state_saver: Default::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };

    let mut sid = LazyStateID::new_unchecked(3);
    let mut mat = None;

    let _ = eoi_rev(&dfa, &mut cache, &input, &mut sid, &mut mat);
}

