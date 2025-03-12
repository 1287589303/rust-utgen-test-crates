// Answer 0

#[test]
fn test_patch_with_splits_state_increases_memory_usage() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::empty(),
        size_limit: Some(100),
    };

    let pattern = String::from("test");
    let compiler = Compiler::new(config, pattern);
    
    let state_id_1 = compiler.add(State::Splits { targets: vec![], reverse: false }).unwrap();
    let state_id_2 = compiler.add(State::Char { target: 0, ch: 'a' }).unwrap();

    // Ensure the initial memory usage is below the limit
    compiler.nfa.borrow_mut().memory_extra = 0;

    // This should increase memory usage above the limit
    compiler.patch(state_id_1, state_id_2).unwrap_err();
}

#[test]
fn test_patch_with_splits_state_and_memory_growth() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::empty(),
        size_limit: Some(100),
    };

    let pattern = String::from("example");
    let compiler = Compiler::new(config, pattern);
    
    let state_id_splits = compiler.add(State::Splits { targets: vec![], reverse: false }).unwrap();
    let state_id_target = compiler.add(State::Match).unwrap();

    // Set initial memory usage
    compiler.nfa.borrow_mut().memory_extra = 50;

    // Add a transition that will increase memory
    compiler.patch(state_id_splits, state_id_target).unwrap();
    
    // Simulate the memory check to ensure the limit will be reached
    compiler.nfa.borrow_mut().memory_extra = 101; // Simulates exceeding the limit
    assert!(compiler.check_size_limit().is_err());
}

#[test]
fn test_patch_on_splits_increments_memory_extra() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::empty(),
        size_limit: None,
    };

    let pattern = String::from("example");
    let compiler = Compiler::new(config, pattern);
    
    let state_id_splits = compiler.add(State::Splits { targets: vec![], reverse: false }).unwrap();
    let state_id_other = compiler.add(State::Char { target: 1, ch: 'b' }).unwrap();

    // Start with some memory usage
    compiler.nfa.borrow_mut().memory_extra = 20;

    let initial_memory_extra = compiler.nfa.borrow().memory_extra;

    // This will modify the targets and increase memory_extra
    compiler.patch(state_id_splits, state_id_other).unwrap();

    let updated_memory_extra = compiler.nfa.borrow().memory_extra;
    assert!(updated_memory_extra > initial_memory_extra);
}

