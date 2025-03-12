// Answer 0

#[test]
fn test_fmt_valid_case() {
    let mut dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![
            Transition { byte: 0, next: StateID(1) }, 
            Transition { byte: 1, next: StateID(2) }
        ],
        starts: vec![StateID(0)],
        min_match_id: StateID(1),
        classes: ByteClasses([0; 256]),
        alphabet_len: 2,
        stride2: 1,
        pateps_offset: 0,
        explicit_slot_start: 2,
    };
    
    let sid = StateID(0);
    
    // Setting up PatternEpsilons to meet precondition where is_empty() is false
    dfa.set_pattern_epsilons(sid, PatternEpsilons(1));

    let _ = format!("{:?}", dfa);
}

#[test]
fn test_fmt_another_case() {
    let mut dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![
            Transition { byte: 0, next: StateID(1) }, 
            Transition { byte: 1, next: StateID(2) }
        ],
        starts: vec![StateID(0)],
        min_match_id: StateID(1),
        classes: ByteClasses([0; 256]),
        alphabet_len: 2,
        stride2: 1,
        pateps_offset: 0,
        explicit_slot_start: 2,
    };
    
    let sid = StateID(1);
    
    dfa.set_pattern_epsilons(sid, PatternEpsilons(2));  

    let _ = format!("{:?}", dfa);
}

