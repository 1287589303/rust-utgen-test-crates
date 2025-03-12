// Answer 0

#[test]
fn test_save_state_valid_id() {
    let mut cache = Cache {
        stack: Vec::new(),
        curr: ActiveStates::default(),
        next: ActiveStates::default(),
        explicit_slots: Vec::new(),
        explicit_slot_len: 0,
    };
    
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 8,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };
    
    let mut lazy = Lazy::new(&dfa, &mut cache);
    let valid_id = LazyStateID(1);
    
    lazy.save_state(valid_id);
}

#[test]
fn test_save_state_boundary_id_zero() {
    let mut cache = Cache {
        stack: Vec::new(),
        curr: ActiveStates::default(),
        next: ActiveStates::default(),
        explicit_slots: Vec::new(),
        explicit_slot_len: 0,
    };
    
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 8,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };
    
    let mut lazy = Lazy::new(&dfa, &mut cache);
    let boundary_id = LazyStateID(0);
    
    lazy.save_state(boundary_id);
}

#[test]
fn test_save_state_boundary_id_max() {
    let mut cache = Cache {
        stack: Vec::new(),
        curr: ActiveStates::default(),
        next: ActiveStates::default(),
        explicit_slots: Vec::new(),
        explicit_slot_len: 0,
    };
    
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 8,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };
    
    let mut lazy = Lazy::new(&dfa, &mut cache);
    let boundary_id = LazyStateID(u32::MAX);
    
    lazy.save_state(boundary_id);
}

#[test]
fn test_save_state_cache_capacity() {
    let mut cache = Cache {
        stack: Vec::new(),
        curr: ActiveStates::default(),
        next: ActiveStates::default(),
        explicit_slots: Vec::new(),
        explicit_slot_len: 1,
    };
    
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 8,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 1,
    };
    
    let mut lazy = Lazy::new(&dfa, &mut cache);
    let valid_id = LazyStateID(1);
    
    lazy.save_state(valid_id);
}

