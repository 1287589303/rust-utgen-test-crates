// Answer 0

#[test]
fn test_fmt_with_no_states() {
    let mut dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![Transition { byte: 0, next: StateID::default() }; 512],
        starts: vec![StateID(1)],
        min_match_id: StateID(0),
        classes: ByteClasses([0; 256]),
        alphabet_len: 1,
        stride2: 9,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };

    // Here, we simulate a situation where the `state_len` is 0.
    // This will ensure that we encounter `index in 0..self.state_len()` is false.
    dfa.set_pattern_epsilons(StateID(1), PatternEpsilons::empty());

    let _ = writeln!(std::fmt::Formatter, "onepass::DFA(");
    for index in 0..dfa.state_len() {
        let sid = StateID::must(index);
        let pateps = dfa.pattern_epsilons(sid);
        if sid == DEAD {
            continue;
        }
        let _ = writeln!(std::fmt::Formatter, "{:06?}", sid.as_usize());
    }
    let _ = writeln!(std::fmt::Formatter, "");
    for (i, &sid) in dfa.starts.iter().enumerate() {
        if i < 1 { continue; }
        let _ = writeln!(std::fmt::Formatter, "START(pattern: {:?}): {:?}", i - 1, sid.as_usize());
    }
    let _ = writeln!(std::fmt::Formatter, "state length: {:?}", dfa.state_len());
    let _ = writeln!(std::fmt::Formatter, "pattern length: {:?}", dfa.pattern_len());
    let _ = writeln!(std::fmt::Formatter, ")");
}

#[test]
fn test_fmt_with_multiple_states() {
    let mut dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![
            Transition { byte: 0, next: StateID(1) },
            Transition { byte: 1, next: StateID(2) },
        ],
        starts: vec![StateID(0), StateID(1)],
        min_match_id: StateID(1),
        classes: ByteClasses([0; 256]),
        alphabet_len: 2,
        stride2: 2,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };

    // Assume that the DFA will have valid pattern transitions
    dfa.set_pattern_epsilons(StateID(1), PatternEpsilons::empty());
    dfa.set_pattern_epsilons(StateID(2), PatternEpsilons::empty());

    let _ = writeln!(std::fmt::Formatter, "onepass::DFA(");
    for index in 0..dfa.state_len() {
        let sid = StateID::must(index);
        let pateps = dfa.pattern_epsilons(sid);
        if sid == DEAD {
            continue;
        }
        let _ = writeln!(std::fmt::Formatter, "{:06?}", sid.as_usize());
    }
    let _ = writeln!(std::fmt::Formatter, "");
    for (i, &sid) in dfa.starts.iter().enumerate() {
        if i == 0 {
            let _ = writeln!(std::fmt::Formatter, "START(ALL): {:?}", sid.as_usize());
        } else {
            let _ = writeln!(std::fmt::Formatter, "START(pattern: {:?}): {:?}", i - 1, sid.as_usize());
        }
    }
    let _ = writeln!(std::fmt::Formatter, "state length: {:?}", dfa.state_len());
    let _ = writeln!(std::fmt::Formatter, "pattern length: {:?}", dfa.pattern_len());
    let _ = writeln!(std::fmt::Formatter, ")");
}

