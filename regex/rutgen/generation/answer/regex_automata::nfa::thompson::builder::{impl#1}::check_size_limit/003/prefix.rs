// Answer 0

#[test]
fn test_check_size_limit_equal_memory_usage() {
    let mut builder = Builder::new();
    let limit = 1024; // Set a size limit greater than 0
    builder.set_size_limit(Some(limit)).unwrap();
    builder.memory_states = 0; // Initialize memory_states
    
    // Add a state to match the limit
    builder.states.push(State::default()); // default will ensure minimal state size
    
    // Adjust the total usage to match the limit
    builder.memory_states = limit - (builder.states.len() * mem::size_of::<State>());
    
    let result = builder.check_size_limit();
    // Check that it returns Ok(())
    result.unwrap();
}

#[test]
fn test_check_size_limit_single_state() {
    let mut builder = Builder::new();
    let limit = 512; // Set a size limit greater than 0
    builder.set_size_limit(Some(limit)).unwrap();
    builder.memory_states = 0; // Initialize memory_states
    
    // Add one state to match the limit
    builder.states.push(State::default()); // default will ensure minimal state size
    
    // Adjust the total usage to match the limit
    builder.memory_states = limit - (builder.states.len() * mem::size_of::<State>());
    
    let result = builder.check_size_limit();
    // Check that it returns Ok(())
    result.unwrap();
}

