// Answer 0

#[test]
fn test_run_with_non_unicode_word_boundary() {
    let nfa = NFA::always_match();
    let mut dfa = dense::OwnedDFA::default();
    let config = Config {
        quit: ByteSet::empty(),
        ..Default::default()
    };

    let mut runner = Runner {
        config,
        nfa: &nfa,
        dfa: &mut dfa,
        builder_states: vec![State::dead()],
        cache: StateMap::default(),
        memory_usage_state: 0,
        sparses: SparseSets::default(),
        stack: vec![],
        scratch_state_builder: StateBuilderEmpty(vec![]),
    };

    let mut uncompiled = vec![];

    // Precondition: self.nfa.look_set_any().contains_word_unicode() is false
    assert!(!runner.nfa.look_set_any().contains_word_unicode());

    // Precondition: self.add_all_starts(&mut uncompiled)? is Ok
    runner.add_all_starts(&mut uncompiled).unwrap();

    // Precondition: let Some(dfa_id) = uncompiled.pop() is true
    let dfa_id = uncompiled.pop().unwrap();

    // Precondition: &unit in &representatives is true
    let unit = runner.dfa.byte_classes().representatives(..).next().unwrap();

    // Precondition: unit.as_u8().map_or(false, |b| self.config.quit.contains(b)) is false
    assert!(!unit.as_u8().map_or(false, |b| runner.config.quit.contains(b)));

    // Precondition: self.cached_state(dfa_id, unit)? returns Err or None
    let result = runner.cached_state(dfa_id, unit);
    assert!(result.is_err());
}

