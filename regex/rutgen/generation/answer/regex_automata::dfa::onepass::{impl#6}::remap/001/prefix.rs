// Answer 0

#[test]
fn test_remap_valid_indices() {
    let mut dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![Transition::new(false, StateID::default(), Epsilons::default()); 4], // example size
        starts: vec![StateID(0)],
        min_match_id: StateID(0),
        classes: ByteClasses([0; 256]),
        alphabet_len: 2,
        stride2: 2,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    
    let map_fn = |id: StateID| StateID(id.0 + 1);
    dfa.remap(map_fn);
}

#[test]
fn test_remap_invalid_beyond_alphabet_len() {
    let mut dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![Transition::new(false, StateID::default(), Epsilons::default()); 4], // example size
        starts: vec![StateID(0)],
        min_match_id: StateID(0),
        classes: ByteClasses([0; 256]),
        alphabet_len: 2,
        stride2: 2,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
  
    let map_fn = |id: StateID| StateID(id.0 + 1);
    dfa.alphabet_len = 2; // still within bounds
    dfa.remap(map_fn);

    dfa.alphabet_len = 3; // exceed bounds
    dfa.remap(map_fn);
}

#[test]
fn test_remap_invalid_start_state_index() {
    let mut dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![Transition::new(false, StateID::default(), Epsilons::default()); 4], 
        starts: vec![],
        min_match_id: StateID(0),
        classes: ByteClasses([0; 256]),
        alphabet_len: 2,
        stride2: 2,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    
    let map_fn = |id: StateID| StateID(id.0 + 1);
    dfa.remap(map_fn);
}

