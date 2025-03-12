// Answer 0

#[test]
fn test_dfa_fmt_empty() {
    use crate::dfa::dense::Flags;
    use crate::{dfa::automaton::Automaton, dfa::dense::DFA, dfa::dense::StateID, dfa::dense::ByteClasses};
    
    let flags = Flags {
        has_empty: false,
        is_utf8: false,
        is_always_start_anchored: false,
    };
    
    let transitions = Transitions {
        sparse: vec![],
        classes: ByteClasses::default(),
        state_len: 0,
        pattern_len: 0,
    };
    
    let start_table = StartTable {
        table: vec![],
        kind: StartKind::Both,
        start_map: StartByteMap::default(),
        stride: 0,
        pattern_len: None,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    
    let dfa = DFA {
        tt: transitions,
        st: start_table,
        special: Special {
            max: StateID(0),
            quit_id: StateID(0),
            min_match: StateID(0),
            max_match: StateID(0),
            min_accel: StateID(0),
            max_accel: StateID(0),
            min_start: StateID(0),
            max_start: StateID(0),
        },
        pre: None,
        quitset: ByteSet::default(),
        flags,
    };
    
    let _ = std::fmt::Debug::fmt(&dfa, &mut std::fmt::Formatter::new());
}

