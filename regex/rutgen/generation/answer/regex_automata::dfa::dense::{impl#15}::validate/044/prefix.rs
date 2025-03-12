// Answer 0

#[test]
fn test_validate_with_valid_states() {
    let transition_table = TransitionTable {
        table: vec![0, 1, 2, 3, 4],
        classes: ByteClasses([0; 256]),
        stride2: 1,
    };

    let special = Special {
        max: StateID(4),
        quit_id: StateID(3),
        min_match: StateID(2),
        max_match: StateID(4),
        min_accel: StateID(1),
        max_accel: StateID(3),
        min_start: StateID(0),
        max_start: StateID(4),
    };

    let dfa = DFA {
        tt: transition_table,
        st: StartTable::new(),
        ms: MatchStates::new(),
        special,
        accels: Accels::new(),
        pre: None,
        quitset: ByteSet::new(),
        flags: Flags::new(),
    };

    let _ = dfa.tt.validate(&dfa);
}

#[test]
fn test_validate_with_non_matching_state_ids() {
    let transition_table = TransitionTable {
        table: vec![0, 1, 2],
        classes: ByteClasses([0; 256]),
        stride2: 1,
    };

    let special = Special {
        max: StateID(2),
        quit_id: StateID(1),
        min_match: StateID(1),
        max_match: StateID(2),
        min_accel: StateID(0),
        max_accel: StateID(2),
        min_start: StateID(0),
        max_start: StateID(2),
    };

    let dfa = DFA {
        tt: transition_table,
        st: StartTable::new(),
        ms: MatchStates::new(),
        special,
        accels: Accels::new(),
        pre: None,
        quitset: ByteSet::new(),
        flags: Flags::new(),
    };

    let _ = dfa.tt.validate(&dfa);
}

