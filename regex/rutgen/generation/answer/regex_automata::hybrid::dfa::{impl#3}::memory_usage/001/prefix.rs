// Answer 0

#[test]
fn test_memory_usage_case_1() {
    let trans = vec![LazyStateID(1), LazyStateID(2)];
    let starts = vec![LazyStateID(1)];
    let states = vec![State { transitions: vec![] }];
    let states_to_id = std::collections::HashMap::new();
    let sparses = SparseSets::new(0);
    let stack = vec![NFAStateID(1)].into_iter().collect::<Vec<_>>();
    let scratch_state_builder = StateBuilderEmpty::new();
    let memory_usage_state = 0;

    let cache = Cache {
        trans,
        starts,
        states,
        states_to_id,
        sparses,
        stack,
        scratch_state_builder,
        memory_usage_state,
        clear_count: 0,
        bytes_searched: 0,
        memory_usage_state: 0,
        progress: None,
    };
    
    let _ = cache.memory_usage();
}

#[test]
fn test_memory_usage_case_2() {
    let trans = vec![LazyStateID(1), LazyStateID(2), LazyStateID(3)];
    let starts = vec![LazyStateID(1), LazyStateID(2)];
    let states = vec![State { transitions: vec![] }, State { transitions: vec![] }];
    let states_to_id = std::collections::HashMap::new();
    let sparses = SparseSets::new(0);
    let stack = vec![NFAStateID(1); 5];
    let scratch_state_builder = StateBuilderEmpty::new();
    let memory_usage_state = 10;

    let cache = Cache {
        trans,
        starts,
        states,
        states_to_id,
        sparses,
        stack,
        scratch_state_builder,
        memory_usage_state,
        clear_count: 3,
        bytes_searched: 100,
        memory_usage_state: 10,
        progress: None,
    };
    
    let _ = cache.memory_usage();
}

#[test]
fn test_memory_usage_boundary_case() {
    let trans = Vec::new();
    let starts = Vec::new();
    let states = Vec::new();
    let states_to_id = std::collections::HashMap::new();
    let sparses = SparseSets::new(0);
    let stack = Vec::new();
    let scratch_state_builder = StateBuilderEmpty::new();
    let memory_usage_state = 0;

    let cache = Cache {
        trans,
        starts,
        states,
        states_to_id,
        sparses,
        stack,
        scratch_state_builder,
        memory_usage_state,
        clear_count: 0,
        bytes_searched: 0,
        memory_usage_state: 0,
        progress: None,
    };
    
    let _ = cache.memory_usage();
}

#[test]
fn test_memory_usage_large_case() {
    let trans = (0..1000).map(|_| LazyStateID(1)).collect::<Vec<_>>();
    let starts = vec![LazyStateID(1), LazyStateID(2), LazyStateID(3)];
    let states = (0..1000).map(|_| State { transitions: vec![] }).collect::<Vec<_>>();
    let states_to_id = std::collections::HashMap::new();
    let sparses = SparseSets::new(100);
    let stack = vec![NFAStateID(1); 100];
    let scratch_state_builder = StateBuilderEmpty::new();
    let memory_usage_state = 500;

    let cache = Cache {
        trans,
        starts,
        states,
        states_to_id,
        sparses,
        stack,
        scratch_state_builder,
        memory_usage_state,
        clear_count: 5,
        bytes_searched: 5000,
        memory_usage_state: 500,
        progress: None,
    };
    
    let _ = cache.memory_usage();
}

