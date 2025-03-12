// Answer 0

#[test]
#[should_panic]
fn test_set_transition_invalid_from() {
    let mut cache = Cache {
        trans: vec![LazyStateID(0); 100],
        starts: vec![],
        states: vec![],
        states_to_id: std::collections::HashMap::new(),
        sparses: SparseSets::new(),
        stack: vec![],
        scratch_state_builder: StateBuilderEmpty::default(),
        state_saver: StateSaver::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };
    
    let dfa = DFA {
        tt: vec![],
        st: vec![],
        ms: MatchStates::default(),
        special: Special::default(),
        accels: Accels::default(),
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags::default(),
    };
    
    let mut lazy_instance = Lazy { dfa: &dfa, cache: &mut cache };
    
    let from = LazyStateID(LazyStateID::MAX + 1);
    let unit = alphabet::Unit::EOI(256);
    let to = LazyStateID(LazyStateID::MAX + 1);
    
    lazy_instance.set_transition(from, unit, to);
}

#[test]
#[should_panic]
fn test_set_transition_invalid_to() {
    let mut cache = Cache {
        trans: vec![LazyStateID(0); 100],
        starts: vec![],
        states: vec![],
        states_to_id: std::collections::HashMap::new(),
        sparses: SparseSets::new(),
        stack: vec![],
        scratch_state_builder: StateBuilderEmpty::default(),
        state_saver: StateSaver::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };
    
    let dfa = DFA {
        tt: vec![],
        st: vec![],
        ms: MatchStates::default(),
        special: Special::default(),
        accels: Accels::default(),
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags::default(),
    };
    
    let mut lazy_instance = Lazy { dfa: &dfa, cache: &mut cache };
    
    let from = LazyStateID(0);
    let unit = alphabet::Unit::EOI(256);
    let to = LazyStateID(LazyStateID::MAX + 1);
    
    lazy_instance.set_transition(from, unit, to);
}

