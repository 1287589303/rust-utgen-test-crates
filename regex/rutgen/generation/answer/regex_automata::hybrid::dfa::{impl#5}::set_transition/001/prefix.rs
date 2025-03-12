// Answer 0

#[test]
fn test_set_transition_valid_ids() {
    let mut cache = Cache {
        trans: vec![LazyStateID(0); 512], // Initialized with enough space
        starts: vec![],
        states: vec![],
        states_to_id: StateMap::new(),
        sparses: SparseSets::new(),
        stack: vec![],
        scratch_state_builder: StateBuilderEmpty::default(),
        state_saver: StateSaver::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };
    
    let byte_classes = ByteClasses([0; 256]);
    let dfa = DFA {
        tt: vec![],
        st: vec![],
        ms: vec![],
        special: Special::default(),
        accels: vec![],
        pre: None,
        quitset: ByteSet::new(),
        flags: Flags::default(),
        classes: byte_classes.clone(),
    };
    
    let mut lazy = Lazy { dfa: &dfa, cache: &mut cache };
    
    let from = LazyStateID::new_unchecked(0);
    let to = LazyStateID::new_unchecked(1);
    let unit = alphabet::Unit::from_byte(0);

    lazy.set_transition(from, unit, to);
}

#[test]
fn test_set_transition_valid_boundary_ids() {
    let mut cache = Cache {
        trans: vec![LazyStateID(0); 512], // Initialized with enough space
        starts: vec![],
        states: vec![],
        states_to_id: StateMap::new(),
        sparses: SparseSets::new(),
        stack: vec![],
        scratch_state_builder: StateBuilderEmpty::default(),
        state_saver: StateSaver::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };
    
    let byte_classes = ByteClasses([0; 256]);
    let dfa = DFA {
        tt: vec![],
        st: vec![],
        ms: vec![],
        special: Special::default(),
        accels: vec![],
        pre: None,
        quitset: ByteSet::new(),
        flags: Flags::default(),
        classes: byte_classes.clone(),
    };
    
    let mut lazy = Lazy { dfa: &dfa, cache: &mut cache };
    
    let from = LazyStateID::new_unchecked(2_u32.pow(31) - 1);
    let to = LazyStateID::new_unchecked(2_u32.pow(31) - 1);
    let unit = alphabet::Unit::from_byte(255);

    lazy.set_transition(from, unit, to);
}

#[test]
#[should_panic]
fn test_set_transition_invalid_from_id() {
    let mut cache = Cache {
        trans: vec![LazyStateID(0); 512],
        starts: vec![],
        states: vec![],
        states_to_id: StateMap::new(),
        sparses: SparseSets::new(),
        stack: vec![],
        scratch_state_builder: StateBuilderEmpty::default(),
        state_saver: StateSaver::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };

    let byte_classes = ByteClasses([0; 256]);
    let dfa = DFA {
        tt: vec![],
        st: vec![],
        ms: vec![],
        special: Special::default(),
        accels: vec![],
        pre: None,
        quitset: ByteSet::new(),
        flags: Flags::default(),
        classes: byte_classes.clone(),
    };

    let mut lazy = Lazy { dfa: &dfa, cache: &mut cache };

    let from = LazyStateID::new_unchecked(2_u32.pow(31)); // Invalid
    let to = LazyStateID::new_unchecked(1);
    let unit = alphabet::Unit::from_byte(0);

    lazy.set_transition(from, unit, to);
}

#[test]
#[should_panic]
fn test_set_transition_invalid_to_id() {
    let mut cache = Cache {
        trans: vec![LazyStateID(0); 512],
        starts: vec![],
        states: vec![],
        states_to_id: StateMap::new(),
        sparses: SparseSets::new(),
        stack: vec![],
        scratch_state_builder: StateBuilderEmpty::default(),
        state_saver: StateSaver::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };

    let byte_classes = ByteClasses([0; 256]);
    let dfa = DFA {
        tt: vec![],
        st: vec![],
        ms: vec![],
        special: Special::default(),
        accels: vec![],
        pre: None,
        quitset: ByteSet::new(),
        flags: Flags::default(),
        classes: byte_classes.clone(),
    };

    let mut lazy = Lazy { dfa: &dfa, cache: &mut cache };

    let from = LazyStateID::new_unchecked(0);
    let to = LazyStateID::new_unchecked(2_u32.pow(31)); // Invalid
    let unit = alphabet::Unit::from_byte(0);

    lazy.set_transition(from, unit, to);
}

