// Answer 0

#[test]
fn test_set_all_transitions_with_min_from() {
    let mut cache = Cache {
        trans: vec![LazyStateID(0); 256],
        starts: vec![LazyStateID(0)],
        states: vec![],
        states_to_id: StateMap::new(),
        sparses: SparseSets::new(),
        stack: vec![],
        scratch_state_builder: StateBuilderEmpty,
        state_saver: StateSaver::new(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };
    
    let classes = ByteClasses([0; 256]);
    let dfa = DFA {
        tt: vec![],
        st: vec![],
        ms: vec![],
        special: Special::default(),
        accels: vec![],
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags::default(),
        classes,
    };
    
    let mut lazy = Lazy {
        dfa: &dfa,
        cache: &mut cache,
    };

    lazy.set_all_transitions(LazyStateID(0), LazyStateID(u32::MAX));
}

#[test]
fn test_set_all_transitions_with_max_from() {
    let mut cache = Cache {
        trans: vec![LazyStateID(0); 256],
        starts: vec![LazyStateID(0)],
        states: vec![],
        states_to_id: StateMap::new(),
        sparses: SparseSets::new(),
        stack: vec![],
        scratch_state_builder: StateBuilderEmpty,
        state_saver: StateSaver::new(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };

    let classes = ByteClasses([0; 256]);
    let dfa = DFA {
        tt: vec![],
        st: vec![],
        ms: vec![],
        special: Special::default(),
        accels: vec![],
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags::default(),
        classes,
    };
    
    let mut lazy = Lazy {
        dfa: &dfa,
        cache: &mut cache,
    };

    lazy.set_all_transitions(LazyStateID(u32::MAX), LazyStateID(u32::MAX));
}

