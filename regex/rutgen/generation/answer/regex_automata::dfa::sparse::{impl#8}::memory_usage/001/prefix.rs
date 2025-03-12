// Answer 0

#[test]
fn test_memory_usage_zero_size_sparse() {
    let sparse: Vec<u8> = vec![];
    let classes = ByteClasses([0; 256]);
    let transitions = Transitions {
        sparse,
        classes,
        state_len: 1,
        pattern_len: 0,
    };
    transitions.memory_usage();
}

#[test]
fn test_memory_usage_one_byte_sparse() {
    let sparse: Vec<u8> = vec![0];
    let classes = ByteClasses([0; 256]);
    let transitions = Transitions {
        sparse,
        classes,
        state_len: 1,
        pattern_len: 0,
    };
    transitions.memory_usage();
}

#[test]
fn test_memory_usage_two_bytes_sparse() {
    let sparse: Vec<u8> = vec![0, 1];
    let classes = ByteClasses([0; 256]);
    let transitions = Transitions {
        sparse,
        classes,
        state_len: 1,
        pattern_len: 0,
    };
    transitions.memory_usage();
}

#[test]
fn test_memory_usage_max_size_sparse() {
    let sparse: Vec<u8> = (0..256).map(|x| x as u8).collect();
    let classes = ByteClasses([0; 256]);
    let transitions = Transitions {
        sparse,
        classes,
        state_len: 1,
        pattern_len: 0,
    };
    transitions.memory_usage();
}

#[test]
fn test_memory_usage_large_sparse() {
    let sparse: Vec<u8> = (0..1000).map(|x| (x % 256) as u8).collect();
    let classes = ByteClasses([0; 256]);
    let transitions = Transitions {
        sparse,
        classes,
        state_len: 1000,
        pattern_len: 10,
    };
    transitions.memory_usage();
}

