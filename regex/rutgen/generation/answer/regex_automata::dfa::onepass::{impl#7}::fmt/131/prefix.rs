// Answer 0

#[test]
fn test_dfa_fmt_empty() {
    use crate::dfa::onepass::DFA;
    use crate::util::int::{StateID, PatternID};
    use crate::util::alphabet::ByteClasses;

    let empty_nfa = NFA::default();
    let empty_classes = ByteClasses([0; 256]);
    let dfa = DFA {
        config: Config::default(),
        nfa: empty_nfa,
        table: vec![],
        starts: vec![],
        min_match_id: StateID::default(),
        classes: empty_classes,
        alphabet_len: 0,
        stride2: 0,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    
    let _ = format!("{:?}", dfa);
}

#[test]
fn test_dfa_fmt_state_len_zero() {
    use crate::dfa::onepass::DFA;
    use crate::util::int::{StateID, PatternID};
    use crate::util::alphabet::ByteClasses;

    let empty_nfa = NFA::default();
    let empty_classes = ByteClasses([0; 256]);
    let dfa = DFA {
        config: Config::default(),
        nfa: empty_nfa,
        table: vec![],
        starts: vec![],
        min_match_id: StateID::default(),
        classes: empty_classes,
        alphabet_len: 0,
        stride2: 0,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    
    let state_len = dfa.state_len(); // This should be 0
    let _ = format!("{:?}", dfa);
    assert_eq!(state_len, 0);
}

#[test]
fn test_dfa_fmt_pattern_len_zero() {
    use crate::dfa::onepass::DFA;
    use crate::util::int::{StateID, PatternID};
    use crate::util::alphabet::ByteClasses;

    let empty_nfa = NFA::default();
    let empty_classes = ByteClasses([0; 256]);
    let dfa = DFA {
        config: Config::default(),
        nfa: empty_nfa,
        table: vec![],
        starts: vec![],
        min_match_id: StateID::default(),
        classes: empty_classes,
        alphabet_len: 0,
        stride2: 0,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    
    let _ = format!("{:?}", dfa);
    let pattern_len = dfa.pattern_len(); // This should return an error or None
    assert!(pattern_len.is_err());
}

#[test]
fn test_dfa_fmt_starts_empty() {
    use crate::dfa::onepass::DFA;
    use crate::util::int::{StateID, PatternID};
    use crate::util::alphabet::ByteClasses;

    let empty_nfa = NFA::default();
    let empty_classes = ByteClasses([0; 256]);
    let dfa = DFA {
        config: Config::default(),
        nfa: empty_nfa,
        table: vec![],
        starts: vec![],
        min_match_id: StateID::default(),
        classes: empty_classes,
        alphabet_len: 0,
        stride2: 0,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    
    let _ = format!("{:?}", dfa);
}

