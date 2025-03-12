// Answer 0

#[test]
fn test_accelerate_fail_due_to_add_limit() {
    let transitions = vec![StateID(1), StateID(2), StateID(3)];
    let state = State {
        id: StateID(0),
        stride2: 4,
        transitions: &transitions,
    };

    let mut classes = ByteClasses::empty();
    classes.set(0u8, 0u8);
    classes.set(1u8, 0u8);
    classes.set(2u8, 0u8);
    
    // Adding one transition with class and id that does not match self.id()
    let result = state.accelerate(&classes);
}

#[test]
fn test_accelerate_with_one_byte_addition() {
    let transitions = vec![StateID(1), StateID(2), StateID(3)];
    let state = State {
        id: StateID(0),
        stride2: 4,
        transitions: &transitions,
    };

    let mut classes = ByteClasses::empty();
    classes.set(0u8, 0u8);
    classes.set(1u8, 0u8);
    classes.set(2u8, 0u8);
    
    let result = state.accelerate(&classes);
}

#[test]
fn test_accelerate_with_multiple_bytes() {
    let transitions = vec![StateID(1), StateID(2), StateID(4)];
    let state = State {
        id: StateID(0),
        stride2: 4,
        transitions: &transitions,
    };

    let mut classes = ByteClasses::empty();
    classes.set(0u8, 1u8);
    classes.set(1u8, 1u8);
    classes.set(2u8, 1u8);
    
    let result = state.accelerate(&classes);
}

