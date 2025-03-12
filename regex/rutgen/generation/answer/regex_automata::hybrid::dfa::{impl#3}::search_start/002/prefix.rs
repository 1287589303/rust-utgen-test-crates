// Answer 0

#[test]
fn test_search_start_progress_taken_with_non_zero_at() {
    let mut cache = Cache {
        trans: Vec::new(),
        starts: Vec::new(),
        states: Vec::new(),
        states_to_id: StateMap::new(),
        sparses: SparseSets { set1: SparseSet::new(), set2: SparseSet::new() },
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty(Vec::new()),
        state_saver: StateSaver::None,
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: Some(SearchProgress { start: 5, at: 10 }),
    };
    cache.search_start(20);
}

#[test]
fn test_search_start_progress_taken_with_zero_at() {
    let mut cache = Cache {
        trans: Vec::new(),
        starts: Vec::new(),
        states: Vec::new(),
        states_to_id: StateMap::new(),
        sparses: SparseSets { set1: SparseSet::new(), set2: SparseSet::new() },
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty(Vec::new()),
        state_saver: StateSaver::None,
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: Some(SearchProgress { start: 0, at: 0 }),
    };
    cache.search_start(0);
}

#[test]
fn test_search_start_progress_taken_at_boundary() {
    let mut cache = Cache {
        trans: Vec::new(),
        starts: Vec::new(),
        states: Vec::new(),
        states_to_id: StateMap::new(),
        sparses: SparseSets { set1: SparseSet::new(), set2: SparseSet::new() },
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty(Vec::new()),
        state_saver: StateSaver::None,
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: Some(SearchProgress { start: usize::MAX - 1, at: usize::MAX }),
    };
    cache.search_start(usize::MAX);
}

#[test]
fn test_search_start_progress_taken_with_large_at() {
    let mut cache = Cache {
        trans: Vec::new(),
        starts: Vec::new(),
        states: Vec::new(),
        states_to_id: StateMap::new(),
        sparses: SparseSets { set1: SparseSet::new(), set2: SparseSet::new() },
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty(Vec::new()),
        state_saver: StateSaver::None,
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: Some(SearchProgress { start: 1, at: 2 }),
    };
    cache.search_start(usize::MAX / 2);
}

