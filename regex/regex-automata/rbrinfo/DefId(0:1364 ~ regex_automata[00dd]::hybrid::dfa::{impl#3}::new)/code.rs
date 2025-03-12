pub fn new(dfa: &DFA) -> Cache {
        let mut cache = Cache {
            trans: alloc::vec![],
            starts: alloc::vec![],
            states: alloc::vec![],
            states_to_id: StateMap::new(),
            sparses: SparseSets::new(dfa.get_nfa().states().len()),
            stack: alloc::vec![],
            scratch_state_builder: StateBuilderEmpty::new(),
            state_saver: StateSaver::none(),
            memory_usage_state: 0,
            clear_count: 0,
            bytes_searched: 0,
            progress: None,
        };
        debug!("pre-init lazy DFA cache size: {}", cache.memory_usage());
        Lazy { dfa, cache: &mut cache }.init_cache();
        debug!("post-init lazy DFA cache size: {}", cache.memory_usage());
        cache
    }