// Answer 0

#[test]
fn test_set_start_state_no_anchored_valid_id() {
    let mut cache = Cache {
        forward: dfa::Cache::default(),
        reverse: dfa::Cache::default(),
    };
    let config = Config::new().starts_for_each_pattern(false);
    let nfa = thompson::NFA::default();
    let dfa = DFA {
        config,
        nfa,
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 1,
    };
    let mut lazy = Lazy { dfa: &dfa, cache: &mut cache };
    let id = LazyStateID(0);
    let start = Start::from_usize(0).unwrap();
    lazy.set_start_state(Anchored::No, start, id);
}

#[test]
fn test_set_start_state_no_anchored_boundary_id() {
    let mut cache = Cache {
        forward: dfa::Cache::default(),
        reverse: dfa::Cache::default(),
    };
    let config = Config::new().starts_for_each_pattern(false);
    let nfa = thompson::NFA::default();
    let dfa = DFA {
        config,
        nfa,
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 1,
    };
    let mut lazy = Lazy { dfa: &dfa, cache: &mut cache };
    let id = LazyStateID(1);
    let start = Start::from_usize(5).unwrap();
    lazy.set_start_state(Anchored::No, start, id);
}

