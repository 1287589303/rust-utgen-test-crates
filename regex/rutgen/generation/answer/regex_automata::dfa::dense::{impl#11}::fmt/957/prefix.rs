// Answer 0

#[test]
fn test_fmt_success() {
    struct TestDFA {
        states: Vec<StateID>,
        starts: Vec<(StateID, Anchored, usize)>,
        ms: MatchStates<Vec<u32>>,
        st: StartTable<Vec<u32>>,
        flags: Flags,
    }

    let state_id_1 = StateID(1);
    let state_id_2 = StateID(2);
    let state_count = 2;
    let start_state_count = 1;
    let pattern_count = 2;

    let dfa = TestDFA {
        states: vec![state_id_1, state_id_2],
        starts: vec![(StateID(0), Anchored::No, 0)],
        ms: MatchStates {
            slices: vec![0, 1, 1, 1],
            pattern_ids: vec![PatternID(0), PatternID(1)],
            pattern_len: 2,
        },
        st: StartTable {
            table: vec![0, 0, 0, 0],
            kind: StartKind::both,
            start_map: StartByteMap::default(),
            stride: 1,
            pattern_len: Some(pattern_count),
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
        flags: Flags {
            has_empty: false,
            is_utf8: true,
            is_always_start_anchored: false,
        },
    };

    let mut formatter = fmt::Formatter::default();

    writeln!(formatter, "dense::DFA(").unwrap();
    for state in &dfa.states {
        fmt_state_indicator(&mut formatter, &dfa, *state).unwrap();
        let id = dfa.to_index(*state);
        write!(formatter, "{:06?}: ", id).unwrap();
        write!(formatter, "State Detail").unwrap(); // Simulating state.fmt(f)
        write!(formatter, "\n").unwrap();
    }
    writeln!(formatter, "").unwrap();
    
    for (i, (start_id, anchored, sty)) in dfa.starts.iter().enumerate() {
        if i % dfa.st.stride == 0 {
            match anchored {
                Anchored::No => writeln!(formatter, "START-GROUP(unanchored)").unwrap(),
                _ => {}
            }
        }
        let id = dfa.to_index(*start_id);
        writeln!(formatter, "  {:?} => {:06?}", sty, id).unwrap();
    }

    if dfa.pattern_len > 1 {
        writeln!(formatter, "").unwrap();
        for i in 0..dfa.ms.len() {
            let id = dfa.ms.match_state_id(&dfa, i);
            let id = dfa.to_index(id);
            write!(formatter, "MATCH({:06?}): ", id).unwrap();
            for (i, &pid) in dfa.ms.pattern_id_slice(i).iter().enumerate() {
                write!(formatter, "{:?}", pid).unwrap();
            }
            writeln!(formatter, "").unwrap();
        }
    }
    writeln!(formatter, "state length: {:?}", dfa.states.len()).unwrap();
    writeln!(formatter, "pattern length: {:?}", dfa.pattern_len).unwrap();
    writeln!(formatter, "flags: {:?}", dfa.flags).unwrap();
    writeln!(formatter, ")").unwrap();
}

