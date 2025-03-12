// Answer 0

#[test]
fn test_try_state_invalid_sparse_state_id() {
    struct SparseTransitions {
        sparse: Vec<u8>,
        classes: ByteClasses,
        state_len: usize,
        pattern_len: usize,
    }

    impl SparseTransitions {
        fn sparse(&self) -> &[u8] {
            &self.sparse
        }
    }

    let sparse = SparseTransitions {
        sparse: vec![0; 10], // has 10 bytes
        classes: ByteClasses([0; 256]),
        state_len: 1,
        pattern_len: 1,
    };
    
    let sp = Special::new();
    let id = StateID(10); // id as usize (10) > sparse().len() (10)

    let _result = sparse.try_state(&sp, id);
}

