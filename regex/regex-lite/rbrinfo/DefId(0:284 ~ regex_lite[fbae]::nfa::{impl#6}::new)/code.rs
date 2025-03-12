fn new(config: Config, pattern: String) -> Compiler {
        let nfa = RefCell::new(NFA {
            pattern,
            states: vec![],
            start: 0,
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: None,
            cap_name_to_index: CaptureNameMap::default(),
            cap_index_to_name: vec![],
            memory_extra: 0,
        });
        Compiler { config, nfa }
    }