// Answer 0

#[test]
fn test_write_to_success() {
    let classes = ByteClasses([0; 256]);
    let stride2 = 1;
    let states = vec![StateID(0), StateID(1), StateID(2), StateID(3)];
    let table = states.clone();
    let transition_table = TransitionTable {
        table,
        classes,
        stride2,
    };

    let nwrite = transition_table.write_to_len();
    let mut buffer = vec![0u8; nwrite];
    
    let _ = transition_table.write_to::<Endian>(&mut buffer);
}

#[test]
#[should_panic]
fn test_write_to_invalid_classes() {
    let classes = ByteClasses([1; 256]);  // Invalid configuration
    let stride2 = 2;
    let states = vec![StateID(0), StateID(1), StateID(2)];
    let table = states.clone();
    let transition_table = TransitionTable {
        table,
        classes,
        stride2,
    };

    let nwrite = transition_table.write_to_len();
    let mut buffer = vec![0u8; nwrite];
    
    let _ = transition_table.write_to::<Endian>(&mut buffer);
}

