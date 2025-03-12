// Answer 0

#[test]
fn test_run_success_path() {
    let nfa = thompson::NFA::always_match();
    let dfa = dense::OwnedDFA::default();
    let mut config = Config::default();
    config.quit = ByteSet::from_bytes(&[0x00]).unwrap().0;

    let runner = Runner {
        config,
        nfa: &nfa,
        dfa: &mut dfa,
        builder_states: vec![],
        cache: StateMap::new(),
        memory_usage_state: 0,
        sparses: SparseSets::default(),
        stack: vec![],
        scratch_state_builder: StateBuilderEmpty::default(),
    };

    let mut uncompiled = vec![];
    runner.add_all_starts(&mut uncompiled).unwrap();
    uncompiled.push(StateID(0)); // Ensure there is a state to pop

    let representatives: Vec<alphabet::Unit> = runner.dfa.byte_classes().representatives(..).collect();
    
    // Ensure there are available representatives
    let unit = representatives[0].clone();
    
    // Mock the state to ensure its cached state is valid and newly created
    runner.cached_state(StateID(0), unit.clone()).unwrap(); 
    
    let result = runner.run();
    
    assert!(result.is_ok());
} 

#[test]
fn test_run_varied_inputs() {
    let nfa = thompson::NFA::never_match();
    let mut dfa = dense::OwnedDFA::default();
    let mut config = Config::default();
    config.quit = ByteSet::from_bytes(&[0x01, 0x02, 0x03]).unwrap().0;

    let mut runner = Runner {
        config,
        nfa: &nfa,
        dfa: &mut dfa,
        builder_states: vec![State::default()],
        cache: StateMap::new(),
        memory_usage_state: 0,
        sparses: SparseSets::default(),
        stack: vec![],
        scratch_state_builder: StateBuilderEmpty::default(),
    };

    let mut uncompiled = vec![];
    runner.add_all_starts(&mut uncompiled).unwrap();
    uncompiled.push(StateID(1)); // Push a dummy state ID

    let representatives: Vec<alphabet::Unit> = runner.dfa.byte_classes().representatives(..).collect();
    let unit = representatives[1].clone(); // Use a second representative

    // Ensure `unit` is not in the quit set
    assert!(!runner.config.quit.contains(unit.as_u8().unwrap()));

    // Ensure there is a valid cached state creation 
    runner.cached_state(StateID(1), unit.clone()).unwrap();

    let result = runner.run();

    assert!(result.is_ok());
} 

#[test]
fn test_run_no_matches() {
    let nfa = thompson::NFA::always_match();
    let mut dfa = dense::OwnedDFA::default();
    let mut config = Config::default();
    config.quit = ByteSet::from_bytes(&[0x80]).unwrap().0; // Only allow byte 0x80

    let mut runner = Runner {
        config,
        nfa: &nfa,
        dfa: &mut dfa,
        builder_states: vec![State::default()],
        cache: StateMap::new(),
        memory_usage_state: 0,
        sparses: SparseSets::default(),
        stack: vec![],
        scratch_state_builder: StateBuilderEmpty::default(),
    };

    let mut uncompiled = vec![];
    runner.add_all_starts(&mut uncompiled).unwrap();
    uncompiled.push(StateID(1)); // Push a dummy state ID

    let representatives: Vec<alphabet::Unit> = runner.dfa.byte_classes().representatives(..).collect();
    let unit = representatives[0].clone(); // Use first representative

    // Ensure `unit` is not in the quit set
    assert!(!runner.config.quit.contains(unit.as_u8().unwrap()));

    // Ensure there is a valid cached state creation
    runner.cached_state(StateID(1), unit.clone()).unwrap();

    let result = runner.run();

    assert!(result.is_ok());
} 

#[test]
fn test_run_error_path() {
    let nfa = thompson::NFA::never_match();
    let mut dfa = dense::OwnedDFA::default();
    let mut config = Config::default();
    config.quit = ByteSet::empty(); // No valid bytes in quit

    let mut runner = Runner {
        config,
        nfa: &nfa,
        dfa: &mut dfa,
        builder_states: vec![State::default()],
        cache: StateMap::new(),
        memory_usage_state: 0,
        sparses: SparseSets::default(),
        stack: vec![],
        scratch_state_builder: StateBuilderEmpty::default(),
    };

    let mut uncompiled = vec![];
    runner.add_all_starts(&mut uncompiled).unwrap();
    uncompiled.push(StateID(1)); // Push a dummy state ID

    let representatives: Vec<alphabet::Unit> = runner.dfa.byte_classes().representatives(..).collect();
    
    // Ensure there are available representatives
    assert!(!representatives.is_empty());

    // Use a representative that is invalid for the quit conditions
    let unit = representatives[0].clone();

    // Ensure it is present in quit, leading to a run potential failure
    let result = runner.run();

    assert!(result.is_ok());
}

