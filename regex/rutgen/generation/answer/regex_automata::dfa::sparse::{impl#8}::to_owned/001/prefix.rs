// Answer 0

#[test]
fn test_to_owned_empty_sparse() {
    let empty_sparse: &[u8] = &[];
    let classes = ByteClasses([0; 256]);
    let state_len = 1;
    let pattern_len = 0;
    
    let transitions = Transitions {
        sparse: empty_sparse,
        classes,
        state_len,
        pattern_len,
    };
    
    let owned = transitions.to_owned();
}

#[test]
fn test_to_owned_non_empty_sparse() {
    let non_empty_sparse: &[u8] = &[1, 2, 3, 4, 5];
    let classes = ByteClasses([1; 256]);
    let state_len = 1;
    let pattern_len = 2;
    
    let transitions = Transitions {
        sparse: non_empty_sparse,
        classes,
        state_len,
        pattern_len,
    };
    
    let owned = transitions.to_owned();
}

#[test]
fn test_to_owned_multiple_state_length() {
    let sparse_data: &[u8] = &[10, 20, 30];
    let classes = ByteClasses([2; 256]);
    let state_len = 2;
    let pattern_len = 1;
    
    let transitions = Transitions {
        sparse: sparse_data,
        classes,
        state_len,
        pattern_len,
    };
    
    let owned = transitions.to_owned();
}

