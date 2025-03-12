// Answer 0

#[test]
#[should_panic]
fn test_match_state_id_no_match_states() {
    let special = Special { max: 0, quit_id: 0, min_match: DEAD, max_match: DEAD, min_accel: 0, max_accel: 0, min_start: 0, max_start: 0 };

    let dfa = DFA {
        tt: TransitionTable { stride2: 0 },
        st: StartTable::default(),
        ms: MatchStates { slices: vec![], pattern_ids: vec![], pattern_len: 0 },
        special,
        accels: Accels::default(),
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags::default(),
    };

    let index = 0;

    let _ = MatchStates::default().match_state_id(&dfa, index);
}

#[test]
#[should_panic]
fn test_match_state_id_no_match_states_boundary() {
    let special = Special { max: 0, quit_id: 0, min_match: DEAD, max_match: DEAD, min_accel: 0, max_accel: 0, min_start: 0, max_start: 0 };

    let dfa = DFA {
        tt: TransitionTable { stride2: 1 },
        st: StartTable::default(),
        ms: MatchStates { slices: vec![], pattern_ids: vec![], pattern_len: 0 },
        special,
        accels: Accels::default(),
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags::default(),
    };

    let index = 0;

    let _ = MatchStates::default().match_state_id(&dfa, index);
}

