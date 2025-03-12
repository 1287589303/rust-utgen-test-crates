// Answer 0

#[test]
fn test_reset_with_valid_onepass() {
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![Transition::default(); 512],
        starts: vec![StateID::default()],
        min_match_id: StateID::default(),
        classes: ByteClasses::default(),
        alphabet_len: 256,
        stride2: 9,
        pateps_offset: 0,
        explicit_slot_start: 2,
    };

    #[cfg(feature = "dfa-onepass")]
    let one_pass_engine = OnePassEngine(Some(onepass::DFA::default()));
    #[cfg(not(feature = "dfa-onepass"))]
    let one_pass_engine = OnePassEngine(None);

    let mut one_pass = OnePass(Some(one_pass_engine));
    let mut one_pass_cache = OnePassCache::none();

    one_pass_cache.reset(&one_pass);
}

#[test]
#[should_panic]
fn test_reset_without_onepass_engine() {
    let mut one_pass_cache = OnePassCache::none();
    let one_pass = OnePass(None);
    one_pass_cache.reset(&one_pass);
}

