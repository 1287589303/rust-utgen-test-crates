// Answer 0

#[test]
fn test_write_to_with_exact_buffer_size() {
    let sparse_data = vec![1, 2, 3, 4, 5, 6];
    let classes = ByteClasses::singletons();
    let transitions = Transitions {
        sparse: &sparse_data,
        classes,
        state_len: 1,
        pattern_len: 1,
    };
    
    let mut buffer = vec![0; transitions.write_to_len()];
    let _ = transitions.write_to::<crate::util::EndianBig>(&mut buffer);
}

#[test]
fn test_write_to_with_valid_classes() {
    let sparse_data = vec![1, 2, 3, 4, 5, 6];
    let classes = ByteClasses::singletons();
    let transitions = Transitions {
        sparse: &sparse_data,
        classes,
        state_len: 2,
        pattern_len: 2,
    };

    let mut buffer = vec![0; transitions.write_to_len()];
    let _ = transitions.write_to::<crate::util::EndianBig>(&mut buffer);
}

#[test]
fn test_write_to_with_state_underflow() {
    let sparse_data = vec![1, 2, 3, 4, 5, 6];
    let classes = ByteClasses::empty();
    let transitions = Transitions {
        sparse: &sparse_data,
        classes,
        state_len: 3,
        pattern_len: 1,
    };

    let mut buffer = vec![0; transitions.write_to_len()];
    let _ = transitions.write_to::<crate::util::EndianBig>(&mut buffer);
}

#[test]
#[should_panic]
fn test_write_to_state_write_failure() {
    let sparse_data = vec![1, 2, 3, 4, 5, 6];
    let classes = ByteClasses::singletons();
    let transitions = Transitions {
        sparse: &sparse_data,
        classes,
        state_len: 4,
        pattern_len: 0,
    };

    let mut buffer = vec![0; transitions.write_to_len() - 1];  // Insufficient buffer size
    let _ = transitions.write_to::<crate::util::EndianBig>(&mut buffer);
}

