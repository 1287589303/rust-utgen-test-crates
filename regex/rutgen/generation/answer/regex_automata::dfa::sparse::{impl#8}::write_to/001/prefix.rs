// Answer 0

#[test]
fn test_write_to_buffer_too_small() {
    use crate::dfa::dense::Flags;

    let transitions = Transitions {
        sparse: vec![0; 10],
        classes: ByteClasses::empty(),
        state_len: 1,
        pattern_len: 0,
    };

    let buffer_size = transitions.write_to_len() - 1; // One less than needed
    let mut dst = vec![0; buffer_size];

    let result = transitions.write_to::<Flags>(&mut dst);
}

#[test]
fn test_write_to_buffer_too_small_multiple_states() {
    use crate::dfa::dense::Flags;

    let transitions = Transitions {
        sparse: vec![0; 20],
        classes: ByteClasses::empty(),
        state_len: 5,
        pattern_len: 2,
    };

    let buffer_size = transitions.write_to_len() - 1; // One less than needed
    let mut dst = vec![0; buffer_size];

    let result = transitions.write_to::<Flags>(&mut dst);
}

#[test]
fn test_write_to_buffer_too_small_empty_transitions() {
    use crate::dfa::dense::Flags;

    let transitions = Transitions {
        sparse: vec![],
        classes: ByteClasses::empty(),
        state_len: 1,
        pattern_len: 0,
    };

    let buffer_size = transitions.write_to_len() - 1; // One less than needed
    let mut dst = vec![0; buffer_size];

    let result = transitions.write_to::<Flags>(&mut dst);
}

