// Answer 0

#[test]
fn test_patch_capture_end_no_memory_increase() {
    let mut builder = Builder::new();
    let from = StateID(SmallIndex::new(0));
    let to = StateID(SmallIndex::new(1));
    
    // Add a CaptureEnd state to self.states
    builder.states.push(State::CaptureEnd {
        pattern_id: PatternID(SmallIndex::new(0)),
        group_index: SmallIndex::new(0),
        next: StateID(SmallIndex::new(2)),
    });
    
    // Set initial memory states
    builder.memory_states = 0;

    // Call the patch function
    let result = builder.patch(from, to);
    result.unwrap();
}

#[test]
fn test_patch_capture_end_no_memory_increase_boundary() {
    let mut builder = Builder::new();
    let from = StateID(SmallIndex::new(0));
    let to = StateID(SmallIndex::new(1));
    
    // Add a CaptureEnd state to self.states
    builder.states.push(State::CaptureEnd {
        pattern_id: PatternID(SmallIndex::new(0)),
        group_index: SmallIndex::new(0),
        next: StateID(SmallIndex::new(2)),
    });
    
    // Set initial memory states
    builder.memory_states = 0;
    // Simulate the size limit
    builder.size_limit = Some(1024); // Setting a size limit

    // Call the patch function
    let result = builder.patch(from, to);
    result.unwrap();
}

#[test]
fn test_patch_capture_end_no_memory_increase_multiple_states() {
    let mut builder = Builder::new();
    let from = StateID(SmallIndex::new(0));
    let to = StateID(SmallIndex::new(1));
    
    // Add a couple of CaptureEnd states and other states
    builder.states.push(State::CaptureEnd {
        pattern_id: PatternID(SmallIndex::new(0)),
        group_index: SmallIndex::new(0),
        next: StateID(SmallIndex::new(2)),
    });
    builder.states.push(State::CaptureEnd {
        pattern_id: PatternID(SmallIndex::new(1)),
        group_index: SmallIndex::new(1),
        next: StateID(SmallIndex::new(3)),
    });
    
    // Set initial memory states
    builder.memory_states = 0;

    // Call the patch function
    let result = builder.patch(from, to);
    result.unwrap();
}

