// Answer 0

#[test]
fn test_memory_usage_zero_states_zero_memory() {
    let builder = Builder::default();
    let usage = builder.memory_usage();
}

#[test]
fn test_memory_usage_counting_states() {
    let mut builder = Builder::default();
    builder.states.push(State { transitions: vec![] });
    let usage = builder.memory_usage();
}

#[test]
fn test_memory_usage_max_states() {
    let mut builder = Builder::default();
    for _ in 0..1000 {
        builder.states.push(State { transitions: vec![] });
    }
    let usage = builder.memory_usage();
}

#[test]
fn test_memory_usage_non_zero_memory() {
    let mut builder = Builder::default();
    builder.memory_states = 512;
    builder.states.push(State { transitions: vec![] });
    let usage = builder.memory_usage();
}

#[test]
fn test_memory_usage_combined() {
    let mut builder = Builder::default();
    builder.memory_states = 1048576;
    for _ in 0..1000 {
        builder.states.push(State { transitions: vec![] });
    }
    let usage = builder.memory_usage();
}

#[test]
fn test_memory_usage_zero_memory_max_states() {
    let mut builder = Builder::default();
    builder.memory_states = 0;
    for _ in 0..1000 {
        builder.states.push(State { transitions: vec![] });
    }
    let usage = builder.memory_usage();
}

#[test]
fn test_memory_usage_non_zero_memory_max_states() {
    let mut builder = Builder::default();
    builder.memory_states = 256;
    for _ in 0..1000 {
        builder.states.push(State { transitions: vec![] });
    }
    let usage = builder.memory_usage();
}

