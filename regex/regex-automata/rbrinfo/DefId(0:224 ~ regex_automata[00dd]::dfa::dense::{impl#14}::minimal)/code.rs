fn minimal(classes: ByteClasses) -> TransitionTable<Vec<u32>> {
        let mut tt = TransitionTable {
            table: vec![],
            classes,
            stride2: classes.stride2(),
        };
        // Two states, regardless of alphabet size, can always fit into u32.
        tt.add_empty_state().unwrap(); // dead state
        tt.add_empty_state().unwrap(); // quit state
        tt
    }