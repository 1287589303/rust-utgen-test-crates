// Answer 0

#[test]
fn test_next_function_valid_match() {
    let pikevm = PikeVM {
        config: Config::default(),
        nfa: NFA::default(),
    };
    let mut cache = Cache {
        stack: Vec::new(),
        curr: ActiveStates::default(),
        next: ActiveStates::default(),
        trans: vec![LazyStateID::default(); 256],
        starts: vec![LazyStateID::default(); 4],
        states: Vec::new(),
        states_to_id: StateMap::default(),
        sparses: SparseSets::default(),
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty::default(),
        state_saver: StateSaver::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };
    let caps = Captures {
        group_info: GroupInfo::default(),
        pid: Some(PatternID::default()),
        slots: vec![None; 10],
    };
    let input = Input::new(b"test input");
    let searcher = Searcher {
        input,
        last_match_end: None,
    };
    let mut find_matches = FindMatches {
        re: &pikevm,
        cache: &mut cache,
        caps,
        it: searcher,
    };

    let _ = find_matches.next();
}

#[test]
fn test_next_function_no_match() {
    let pikevm = PikeVM {
        config: Config::default(),
        nfa: NFA::default(),
    };
    let mut cache = Cache {
        stack: Vec::new(),
        curr: ActiveStates::default(),
        next: ActiveStates::default(),
        trans: vec![LazyStateID::default(); 256],
        starts: vec![LazyStateID::default(); 4],
        states: Vec::new(),
        states_to_id: StateMap::default(),
        sparses: SparseSets::default(),
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty::default(),
        state_saver: StateSaver::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };
    let caps = Captures {
        group_info: GroupInfo::default(),
        pid: None,
        slots: vec![None; 10],
    };
    let input = Input::new(b"no match here");
    let searcher = Searcher {
        input,
        last_match_end: None,
    };
    let mut find_matches = FindMatches {
        re: &pikevm,
        cache: &mut cache,
        caps,
        it: searcher,
    };

    let _ = find_matches.next();
}

#[test]
fn test_next_function_with_empty_input() {
    let pikevm = PikeVM {
        config: Config::default(),
        nfa: NFA::default(),
    };
    let mut cache = Cache {
        stack: Vec::new(),
        curr: ActiveStates::default(),
        next: ActiveStates::default(),
        trans: vec![LazyStateID::default(); 256],
        starts: vec![LazyStateID::default(); 4],
        states: Vec::new(),
        states_to_id: StateMap::default(),
        sparses: SparseSets::default(),
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty::default(),
        state_saver: StateSaver::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };
    let caps = Captures {
        group_info: GroupInfo::default(),
        pid: None,
        slots: vec![None; 10],
    };
    let input = Input::new(b"");
    let searcher = Searcher {
        input,
        last_match_end: None,
    };
    let mut find_matches = FindMatches {
        re: &pikevm,
        cache: &mut cache,
        caps,
        it: searcher,
    };

    let _ = find_matches.next();
}

