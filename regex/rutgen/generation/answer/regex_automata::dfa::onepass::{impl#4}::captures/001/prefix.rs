// Answer 0

#[test]
fn test_captures_with_no_anchored() {
    let haystack = b"Bruce Springsteen";
    let mut input = Input::new(&haystack).set_anchored(Anchored::No);
    let mut cache = Cache {
        explicit_slots: vec![None; 2],
        explicit_slot_len: 2,
    };
    let mut caps = Captures {
        group_info: GroupInfo::default(),
        pid: None,
        slots: vec![None; 2],
    };
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![],
        starts: vec![],
        min_match_id: StateID::default(),
        classes: ByteClasses([0; 256]),
        alphabet_len: 0,
        stride2: 0,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };

    dfa.captures(&mut cache, input, &mut caps);
}

#[test]
#[should_panic]
fn test_captures_with_no_anchored_pattern() {
    let haystack = b"Bruce Springsteen";
    let mut input = Input::new(&haystack).set_anchored(Anchored::No);
    let mut cache = Cache {
        explicit_slots: vec![None; 2],
        explicit_slot_len: 2,
    };
    let mut caps = Captures {
        group_info: GroupInfo::default(),
        pid: None,
        slots: vec![None; 2],
    };
    let dfa = DFA {
        config: Config {
            match_kind: Some(MatchKind::Anchored),
            ..Config::default()
        },
        nfa: NFA::default(),
        table: vec![],
        starts: vec![],
        min_match_id: StateID::default(),
        classes: ByteClasses([0; 256]),
        alphabet_len: 0,
        stride2: 0,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };

    dfa.captures(&mut cache, input, &mut caps);
}

