// Answer 0

#[test]
fn test_remap_union_single_alternate() {
    let state_id_0 = StateID(SmallIndex(0));
    let state_id_1 = StateID(SmallIndex(1));
    
    let mut state = State::Union {
        alternates: Box::new([state_id_0]),
    };
    
    let remap = [state_id_1];
    
    state.remap(&remap);
}

#[test]
fn test_remap_union_multiple_alternates() {
    let state_id_0 = StateID(SmallIndex(0));
    let state_id_1 = StateID(SmallIndex(1));
    let state_id_2 = StateID(SmallIndex(2));
    
    let mut state = State::Union {
        alternates: Box::new([state_id_0, state_id_1]),
    };
    
    let remap = [state_id_2, state_id_1];
    
    state.remap(&remap);
}

#[test]
fn test_remap_union_no_alternates() {
    let mut state = State::Union {
        alternates: Box::new([]),
    };
    
    let remap: &[StateID] = &[];
    
    state.remap(remap);
}

#[test]
#[should_panic]
fn test_remap_union_invalid_remap() {
    let state_id_0 = StateID(SmallIndex(0));
    
    let mut state = State::Union {
        alternates: Box::new([state_id_0]),
    };
    
    let remap = []; // Empty remap, invalid for a single alternate
    
    state.remap(&remap);
}

