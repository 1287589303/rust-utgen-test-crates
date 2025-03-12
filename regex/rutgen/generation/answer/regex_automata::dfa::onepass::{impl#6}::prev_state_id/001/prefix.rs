// Answer 0

#[test]
fn test_prev_state_id_dead() {
    let id = DEAD;
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![],
        starts: vec![],
        min_match_id: StateID::new_unchecked(1),
        classes: ByteClasses([0; 256]),
        alphabet_len: 0,
        stride2: 0,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    let result = dfa.prev_state_id(id);
}

#[test]
fn test_prev_state_id_first_state() {
    let id = StateID::new_unchecked(0);
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![],
        starts: vec![],
        min_match_id: StateID::new_unchecked(1),
        classes: ByteClasses([0; 256]),
        alphabet_len: 0,
        stride2: 0,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    let result = dfa.prev_state_id(id);
}

