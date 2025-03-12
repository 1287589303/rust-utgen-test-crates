fn new(config: Config, nfa: &'a NFA) -> InternalBuilder<'a> {
        let classes = if !config.get_byte_classes() {
            // A one-pass DFA will always use the equivalence class map, but
            // enabling this option is useful for debugging. Namely, this will
            // cause all transitions to be defined over their actual bytes
            // instead of an opaque equivalence class identifier. The former is
            // much easier to grok as a human.
            ByteClasses::singletons()
        } else {
            nfa.byte_classes().clone()
        };
        // Normally a DFA alphabet includes the EOI symbol, but we don't need
        // that in the one-pass DFA since we handle look-around explicitly
        // without encoding it into the DFA. Thus, we don't need to delay
        // matches by 1 byte. However, we reuse the space that *would* be used
        // by the EOI transition by putting match information there (like which
        // pattern matches and which look-around assertions need to hold). So
        // this means our real alphabet length is 1 fewer than what the byte
        // classes report, since we don't use EOI.
        let alphabet_len = classes.alphabet_len().checked_sub(1).unwrap();
        let stride2 = classes.stride2();
        let dfa = DFA {
            config: config.clone(),
            nfa: nfa.clone(),
            table: vec![],
            starts: vec![],
            // Since one-pass DFAs have a smaller state ID max than
            // StateID::MAX, it follows that StateID::MAX is a valid initial
            // value for min_match_id since no state ID can ever be greater
            // than it. In the case of a one-pass DFA with no match states, the
            // min_match_id will keep this sentinel value.
            min_match_id: StateID::MAX,
            classes: classes.clone(),
            alphabet_len,
            stride2,
            pateps_offset: alphabet_len,
            // OK because PatternID::MAX*2 is guaranteed not to overflow.
            explicit_slot_start: nfa.pattern_len().checked_mul(2).unwrap(),
        };
        InternalBuilder {
            dfa,
            uncompiled_nfa_ids: vec![],
            nfa_to_dfa_id: vec![DEAD; nfa.states().len()],
            stack: vec![],
            seen: SparseSet::new(nfa.states().len()),
            matched: false,
            config,
            nfa,
            classes,
        }
    }