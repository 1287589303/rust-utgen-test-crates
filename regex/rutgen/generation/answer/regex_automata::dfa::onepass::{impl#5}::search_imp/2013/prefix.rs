// Answer 0

#[test]
fn test_search_imp_case_1() {
    let haystack = b"test haystack";
    let mut slots = vec![None; 10];
    let mut cache = Cache::new();
    let input = Input::new(&haystack[0..]).set_range(0..haystack.len()).set_anchored(Anchored::No).set_earliest(true);

    let dfa = DFA {
        config: Config::default()
            .match_kind(MatchKind::LeftmostFirst)
            .starts_for_each_pattern(Some(true)),
        nfa: NFA::always_match(),
        table: Vec::new(),
        starts: vec![StateID::default()],
        min_match_id: StateID(1),
        classes: ByteClasses([0; 256]),
        alphabet_len: 256,
        stride2: 8,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };

    dfa.search_imp(&mut cache, &input, &mut slots).unwrap();
}

#[test]
fn test_search_imp_case_2() {
    let haystack = b"another test haystack";
    let mut slots = vec![None; 15];
    let mut cache = Cache::new();
    let input = Input::new(&haystack[0..]).set_range(0..haystack.len()).set_anchored(Anchored::No).set_earliest(true);

    let dfa = DFA {
        config: Config::default()
            .match_kind(MatchKind::LeftmostFirst)
            .starts_for_each_pattern(Some(false)),
        nfa: NFA::never_match(),
        table: Vec::new(),
        starts: vec![StateID::default()],
        min_match_id: StateID(1),
        classes: ByteClasses([0; 256]),
        alphabet_len: 256,
        stride2: 8,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };

    dfa.search_imp(&mut cache, &input, &mut slots).unwrap();
}

