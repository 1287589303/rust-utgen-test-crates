// Answer 0

#[test]
fn test_clear_cache_with_progress_and_state_saver() {
    let mut cache = Cache {
        capmatches: Captures::new(),
        pikevm: Default::default(),
        backtrack: Default::default(),
        onepass: Default::default(),
        hybrid: Default::default(),
        revhybrid: Default::default(),
    };

    let progress = SearchProgress { start: 0, at: 1 };
    cache.progress = Some(progress);
    
    let valid_state = State(Arc::new([0u8, 1, 2].into()));
    let valid_id = LazyStateID(1);
    cache.state_saver = StateSaver::ToSave { id: valid_id, state: valid_state };

    let dfa = DFA {
        tt: Default::default(),
        st: Default::default(),
        ms: Default::default(),
        special: Default::default(),
        accels: Default::default(),
        pre: None,
        quitset: ByteSet::new(),
        flags: Default::default(),
    };

    let mut lazy = Lazy { dfa: &dfa, cache: &mut cache };
    
    lazy.clear_cache();
} 

#[test]
fn test_clear_cache_with_saved_state_non_sentinel() {
    let mut cache = Cache {
        capmatches: Captures::new(),
        pikevm: Default::default(),
        backtrack: Default::default(),
        onepass: Default::default(),
        hybrid: Default::default(),
        revhybrid: Default::default(),
    };

    let progress = SearchProgress { start: 0, at: 2 };
    cache.progress = Some(progress);
    
    let valid_state = State(Arc::new([3u8, 4, 5].into()));
    let valid_id = LazyStateID(2);
    cache.state_saver = StateSaver::ToSave { id: valid_id, state: valid_state };

    let dfa = DFA {
        tt: Default::default(),
        st: Default::default(),
        ms: Default::default(),
        special: Default::default(),
        accels: Default::default(),
        pre: None,
        quitset: ByteSet::new(),
        flags: Default::default(),
    };

    let mut lazy = Lazy { dfa: &dfa, cache: &mut cache };
    
    lazy.clear_cache();
}

