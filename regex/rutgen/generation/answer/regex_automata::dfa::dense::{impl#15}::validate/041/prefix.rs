// Answer 0

#[test]
fn test_validate_special_state_not_found() {
    let transition_table = TransitionTable {
        table: vec![],
        classes: ByteClasses([0; 256]),
        stride2: 1,
    };
    let special = Special {
        max: 0,
        quit_id: 1,
        min_match: 2,
        max_match: 3,
        min_accel: 4,
        max_accel: 5,
        min_start: 6,
        max_start: 7,
    };
    let dfa = DFA {
        tt: transition_table.clone(),
        st: vec![],
        ms: vec![],
        special,
        accels: vec![],
        pre: None,
        quitset: ByteSet([0; 256]),
        flags: Flags::default(),
    };

    let result = transition_table.validate(&dfa);
}

#[test]
fn test_validate_transition_ids() {
    let transition_table = TransitionTable {
        table: vec![StateID(SmallIndex(0)), StateID(SmallIndex(1))],
        classes: ByteClasses([0; 256]),
        stride2: 1,
    };
    let special = Special {
        max: 1,
        quit_id: 2,
        min_match: 3,
        max_match: 4,
        min_accel: 5,
        max_accel: 6,
        min_start: 7,
        max_start: 8,
    };
    let dfa = DFA {
        tt: transition_table.clone(),
        st: vec![],
        ms: vec![],
        special,
        accels: vec![],
        pre: None,
        quitset: ByteSet([0; 256]),
        flags: Flags::default(),
    };

    let state = State {
        id: StateID(SmallIndex(0)),
        stride2: 1,
        transitions: &[StateID(SmallIndex(1))],
    };

    let result = transition_table.validate(&dfa);
}

