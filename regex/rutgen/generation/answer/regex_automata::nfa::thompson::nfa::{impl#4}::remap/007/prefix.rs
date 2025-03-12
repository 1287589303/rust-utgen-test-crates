// Answer 0

#[test]
fn test_remap_state_look_valid_mapping() {
    let state_id_next = StateID(SmallIndex(1));
    let remap: Vec<StateID> = vec![StateID(SmallIndex(0)), StateID(SmallIndex(2)), StateID(SmallIndex(3))];
    
    let mut state = State::Look {
        look: Look::Start,
        next: state_id_next,
    };

    state.remap(&remap);
}

#[test]
fn test_remap_state_look_boundary_mapping() {
    let state_id_next = StateID(SmallIndex(2));
    let remap: Vec<StateID> = vec![StateID(SmallIndex(0)), StateID(SmallIndex(1))];
    
    let mut state = State::Look {
        look: Look::End,
        next: state_id_next,
    };

    state.remap(&remap);
}

#[test]
#[should_panic]
fn test_remap_state_look_invalid_next_mapping() {
    let state_id_next = StateID(SmallIndex(5));
    let remap: Vec<StateID> = vec![StateID(SmallIndex(0)), StateID(SmallIndex(1)), StateID(SmallIndex(2))];
    
    let mut state = State::Look {
        look: Look::StartLF,
        next: state_id_next,
    };

    state.remap(&remap);
}

