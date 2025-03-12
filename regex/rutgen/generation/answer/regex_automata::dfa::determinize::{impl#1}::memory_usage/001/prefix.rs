// Answer 0

#[test]
fn test_memory_usage_empty() {
    let config = Config::default();
    let nfa = Box::new(thompson::NFA::new());
    let mut dfa = dense::OwnedDFA::default();
    let runner = Runner {
        config,
        nfa: &nfa,
        dfa: &mut dfa,
        builder_states: Vec::new(),
        cache: StateMap::default(),
        memory_usage_state: 0,
        sparses: SparseSets::default(),
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty::new(),
    };
    let _ = runner.memory_usage();
}

#[test]
fn test_memory_usage_one_state() {
    let config = Config::default();
    let nfa = Box::new(thompson::NFA::new());
    let mut dfa = dense::OwnedDFA::default();
    let state = State {
        id: StateID(0),
        transitions: &[],
    };
    let runner = Runner {
        config,
        nfa: &nfa,
        dfa: &mut dfa,
        builder_states: vec![state],
        cache: StateMap::default(),
        memory_usage_state: 10,
        sparses: SparseSets::default(),
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty::new(),
    };
    let _ = runner.memory_usage();
}

#[test]
fn test_memory_usage_multiple_states() {
    let config = Config::default();
    let nfa = Box::new(thompson::NFA::new());
    let mut dfa = dense::OwnedDFA::default();
    let states: Vec<State> = (0..10).map(|i| State { id: StateID(i), transitions: &[] }).collect();
    let runner = Runner {
        config,
        nfa: &nfa,
        dfa: &mut dfa,
        builder_states: states,
        cache: StateMap::default(),
        memory_usage_state: 50,
        sparses: SparseSets::default(),
        stack: vec![0; 5],
        scratch_state_builder: StateBuilderEmpty::new(),
    };
    let _ = runner.memory_usage();
}

#[test]
fn test_memory_usage_large_variations() {
    let config = Config::default();
    let nfa = Box::new(thompson::NFA::new());
    let mut dfa = dense::OwnedDFA::default();
    let states: Vec<State> = (0..1000).map(|i| State { id: StateID(i), transitions: &[] }).collect();
    let runner = Runner {
        config,
        nfa: &nfa,
        dfa: &mut dfa,
        builder_states: states,
        cache: StateMap::default(),
        memory_usage_state: 4096,
        sparses: SparseSets::default(),
        stack: vec![0; 100],
        scratch_state_builder: StateBuilderEmpty::new(),
    };
    let _ = runner.memory_usage();
}

#[test]
fn test_memory_usage_with_cache() {
    let config = Config::default();
    let nfa = Box::new(thompson::NFA::new());
    let mut dfa = dense::OwnedDFA::default();
    let states: Vec<State> = (0..50).map(|i| State { id: StateID(i), transitions: &[] }).collect();
    let runner = Runner {
        config,
        nfa: &nfa,
        dfa: &mut dfa,
        builder_states: states,
        cache: StateMap::from_iter((0..5).map(|i| (State(i), StateID(i)))),
        memory_usage_state: 100,
        sparses: SparseSets::default(),
        stack: vec![0; 20],
        scratch_state_builder: StateBuilderEmpty::new(),
    };
    let _ = runner.memory_usage();
}

