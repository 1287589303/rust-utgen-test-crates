// Answer 0

#[test]
fn test_next_state_id_valid() {
    struct TestTable {
        table: Vec<u32>,
        classes: ByteClasses,
        stride2: usize,
    }

    let valid_stride2 = 3; // Example: power of 2, so stride of 8
    let dfa = TestTable {
        table: vec![0; 8], // Placeholder for actual size, must match stride
        classes: ByteClasses([0; 256]),
        stride2: valid_stride2,
    };
    
    let state_id_zero = StateID(0);
    let next_id_zero = dfa.next_state_id(state_id_zero);
    
    let state_id_max = StateID((1 << valid_stride2) - 1);
    let next_id_max = dfa.next_state_id(state_id_max);
}

#[test]
fn test_next_state_id_boundary() {
    struct TestTable {
        table: Vec<u32>,
        classes: ByteClasses,
        stride2: usize,
    }

    let valid_stride2 = 4; // Example: power of 2, so stride of 16
    let dfa = TestTable {
        table: vec![0; 16], // Placeholder for actual size, must match stride
        classes: ByteClasses([0; 256]),
        stride2: valid_stride2,
    };

    let state_id_boundary = StateID((1 << valid_stride2) - 1);
    let next_id_boundary = dfa.next_state_id(state_id_boundary);
}

