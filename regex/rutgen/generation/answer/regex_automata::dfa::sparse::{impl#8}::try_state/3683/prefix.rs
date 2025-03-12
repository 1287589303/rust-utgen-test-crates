// Answer 0

#[test]
fn test_try_state_invalid_pattern_ids() {
    struct TestTransitions {
        sparse: Vec<u8>,
        classes: ByteClasses,
        state_len: usize,
        pattern_len: usize,
    }

    impl TestTransitions {
        fn sparse(&self) -> &[u8] {
            &self.sparse
        }

        fn id_len(&self) -> usize {
            std::mem::size_of::<StateID>()
        }
    }

    let id = StateID(0.into());
    let sp = Special::new();

    let transitions = TestTransitions {
        sparse: vec![
            0, 1, // ntrans (0)
            0, // (not a match)
            0, // input range placeholder
            0, // next state ID placeholder
            0, // accelerator length
        ],
        classes: ByteClasses([0; 256]),
        state_len: 1,
        pattern_len: 0,
    };

    let result = transitions.try_state(&sp, id);
}
  
#[test]
fn test_try_state_invalid_transition_length() {
    struct TestTransitions {
        sparse: Vec<u8>,
        classes: ByteClasses,
        state_len: usize,
        pattern_len: usize,
    }

    impl TestTransitions {
        fn sparse(&self) -> &[u8] {
            &self.sparse
        }

        fn id_len(&self) -> usize {
            std::mem::size_of::<StateID>()
        }
    }

    let id = StateID(1.into());
    let sp = Special::new();

    let transitions = TestTransitions {
        sparse: vec![
            0, 1, // ntrans (257)
            0, // (not a match)
            0, 0, // valid input range
            1, 0, 0, 0, // valid next state ID
            1, // accelerator length
        ],
        classes: ByteClasses([0; 256]),
        state_len: 1,
        pattern_len: 0,
    };

    let result = transitions.try_state(&sp, id);
}

