// Answer 0

#[test]
fn test_try_state_invalid_id() {
    let id = StateID(257); // bound id.as_usize() == self.sparse().len()
    let state_data: &[u8] = &[];
    let byte_classes = ByteClasses([0; 256]);
    let transitions = Transitions {
        sparse: state_data,
        classes: byte_classes,
        state_len: 1,
        pattern_len: 0,
    };
    let special = Special { 
        max: StateID(256), 
        quit_id: StateID(1), 
        min_match: StateID(2), 
        max_match: StateID(3),
        min_accel: StateID(2), 
        max_accel: StateID(255),
        min_start: StateID(0),
        max_start: StateID(1),
    };

    let _result = transitions.try_state(&special, id);
}

#[test]
fn test_try_state_valid_state() {
    let id = StateID(0);
    let state_data: &[u8] = &[0, 0]; // for ntrans == 0 (2 bytes for u16)
    let byte_classes = ByteClasses([0; 256]);
    let transitions = Transitions {
        sparse: state_data,
        classes: byte_classes,
        state_len: 1,
        pattern_len: 0,
    };
    let special = Special { 
        max: StateID(256), 
        quit_id: StateID(1), 
        min_match: StateID(2), 
        max_match: StateID(3),
        min_accel: StateID(2), 
        max_accel: StateID(255),
        min_start: StateID(0),
        max_start: StateID(1),
    };

    let _result = transitions.try_state(&special, id);
}

